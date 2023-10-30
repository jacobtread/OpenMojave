use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID, LocalizedString};
use crate::error::Error;
use crate::fields::{
    DestructionData, Model, ObjectBounds, ScriptList, EDID, FNAM, FULL, KSIZ, KWDA, OBND, PFIG,
    PFPC, PNAM, RNAM, SNAM, VMAD,
};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"FLOR")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FLOR {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,

    #[br(calc(localized))]
    #[bw(ignore)]
    pub localized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flora {
    pub header: RecordHeader,
    pub edid: String,
    pub scripts: Option<ScriptList>,
    pub bounds: ObjectBounds,
    pub full_name: LocalizedString,
    pub model: Model,
    pub destruction_data: Option<DestructionData>,
    pub keywords: Vec<FormID>,
    pub unknown: u32,
    pub activate_text: Option<LocalizedString>,
    pub flags: u16,
    pub ingredient: Option<FormID>,
    pub pickup_sound: Option<FormID>,
    pub percent_chance: [u8; 4],
}

impl fmt::Display for Flora {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Flora ({})", self.edid)
    }
}

impl TryFrom<FLOR> for Flora {
    type Error = Error;

    fn try_from(raw: FLOR) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let scripts = VMAD::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let bounds = OBND::read(&mut cursor)?.try_into()?;
        let full_name = match (FULL::read(&mut cursor)?, raw.localized) {
            (f, true) => LocalizedString::Localized(f.try_into()?),
            (z, false) => LocalizedString::ZString(z.try_into()?),
        };
        let model = Model::load(&mut cursor, raw.header.internal_version)?;
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
        let unknown = PNAM::read(&mut cursor)?.try_into()?;
        let activate_text = match (RNAM::read(&mut cursor), raw.localized) {
            (Ok(f), true) => Some(LocalizedString::Localized(f.try_into()?)),
            (Ok(z), false) => Some(LocalizedString::ZString(z.try_into()?)),
            (Err(_), _) => None,
        };
        let flags = FNAM::read(&mut cursor)?.try_into()?;
        let ingredient = PFIG::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let pickup_sound = SNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let percent_chance = PFPC::read(&mut cursor)?.try_into()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            scripts,
            bounds,
            full_name,
            model,
            destruction_data,
            keywords,
            unknown,
            activate_text,
            flags,
            ingredient,
            pickup_sound,
            percent_chance,
        })
    }
}
