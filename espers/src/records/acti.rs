use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID, LocalizedString};
use crate::error::Error;
use crate::fields::{
    AlternateTextures, DestructionData, ObjectBounds, ScriptList, Textures, EDID, FNAM, FULL, KNAM,
    KSIZ, KWDA, MODL, MODS, MODT, OBND, PNAM, RNAM, SNAM, VMAD, VNAM, WNAM,
};
use crate::string_table::StringTable;
use binrw::{binrw, BinRead};
use rgb::RGBA8;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"ACTI")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACTI {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,

    #[br(calc(localized))]
    #[bw(ignore)]
    pub localized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activator {
    pub header: RecordHeader,
    pub edid: String,
    pub scripts: Option<ScriptList>,
    pub bounds: ObjectBounds,
    pub full_name: Option<LocalizedString>,
    pub model_filename: Option<String>,
    pub model_textures: Option<Textures>,
    pub alternate_textures: Option<AlternateTextures>,
    pub destruction_data: Option<DestructionData>,
    pub keywords: Vec<FormID>,
    pub marker_color: Option<RGBA8>,
    pub sound_looping: Option<FormID>,
    pub sound_activation: Option<FormID>,
    pub water_type: Option<FormID>,
    pub activate_text_override: Option<LocalizedString>,
    pub flags: Option<u16>,
    pub interaction_keyword: Option<FormID>,
}

impl Activator {
    pub fn localize(&mut self, string_table: &StringTable) {
        if let Some(LocalizedString::Localized(l)) = self.full_name {
            if let Some(s) = string_table.get_string(&l) {
                self.full_name = Some(LocalizedString::ZString(s.clone()));
            }
        }
    }
}

impl fmt::Display for Activator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Activator ({})", self.edid)
    }
}

impl TryFrom<ACTI> for Activator {
    type Error = Error;

    fn try_from(raw: ACTI) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let scripts = VMAD::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let bounds = OBND::read(&mut cursor)?.try_into()?;
        let full_name = match (FULL::read(&mut cursor), raw.localized) {
            (Ok(f), true) => Some(LocalizedString::Localized(f.try_into()?)),
            (Ok(z), false) => Some(LocalizedString::ZString(z.try_into()?)),
            (Err(_), _) => None,
        };
        let model_filename = MODL::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let model_textures = MODT::read(&mut cursor)
            .ok()
            .map(|modt| Textures::load(modt, raw.header.internal_version))
            .transpose()?;
        let alternate_textures = MODS::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let destruction_data = DestructionData::load(&mut cursor)?;
        let keyword_count: Option<u32> = KSIZ::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let mut keywords = Vec::new();

        if let Some(kc) = keyword_count {
            for _ in 0..kc {
                // It's actually only up to keyword count
                if let Ok(kwda) = KWDA::read(&mut cursor) {
                    keywords.push(FormID::read_le(&mut Cursor::new(kwda.data)).unwrap());
                }
            }
        }

        let marker_color = PNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let sound_looping = SNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        while let Ok(_) = SNAM::read(&mut cursor) {}

        let sound_activation = VNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let water_type = WNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let activate_text_override = match (RNAM::read(&mut cursor), raw.localized) {
            (Ok(f), true) => Some(LocalizedString::Localized(f.try_into()?)),
            (Ok(z), false) => Some(LocalizedString::ZString(z.try_into()?)),
            (Err(_), _) => None,
        };
        let flags = FNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let interaction_keyword = KNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            scripts,
            bounds,
            full_name,
            model_filename,
            model_textures,
            alternate_textures,
            destruction_data,
            keywords,
            marker_color,
            sound_looping,
            sound_activation,
            water_type,
            activate_text_override,
            flags,
            interaction_keyword,
        })
    }
}
