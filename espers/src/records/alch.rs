use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID, LocalizedString};
use crate::error::Error;
use crate::fields::{
    AlternateTextures, Condition, EffectItem, ObjectBounds, Unknown4, CTDA, DATA, EDID, EFID, EFIT,
    ENIT, FULL, ICON, KSIZ, KWDA, MICO, MODL, MODS, MODT, OBND, YNAM, ZNAM,
};
use crate::string_table::StringTables;
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"ALCH")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ALCH {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,

    #[br(calc(localized))]
    #[bw(ignore)]
    pub localized: bool,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnchantedItem {
    pub potion_value: u32,
    pub flags: u32,
    pub addiction: FormID,
    pub addiction_chance: u32,
    pub use_sound: FormID,
}

impl TryFrom<ENIT> for EnchantedItem {
    type Error = Error;

    fn try_from(raw: ENIT) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alchemy {
    pub header: RecordHeader,
    pub edid: String,
    pub bounds: ObjectBounds,
    pub full_name: Option<LocalizedString>,
    pub keywords: Vec<FormID>,
    pub model_filename: Option<String>,
    pub model_textures: Vec<Unknown4>,
    pub alternate_textures: Option<AlternateTextures>,
    pub icon: Option<String>,
    pub message_icon: Option<String>,
    pub pickup_sound: Option<FormID>,
    pub drop_sound: Option<FormID>,
    pub weight: f32,
    pub item: EnchantedItem,
    pub effects: Vec<(FormID, EffectItem, Vec<Condition>)>,
}

impl Alchemy {
    pub fn localize(&mut self, string_table: &StringTables) {
        if let Some(LocalizedString::Localized(l)) = self.full_name {
            if let Some(s) = string_table.get_string(&l) {
                self.full_name = Some(LocalizedString::ZString(s.clone()));
            }
        }
    }
}

impl fmt::Display for Alchemy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alchemy ({})", self.edid)
    }
}

impl TryFrom<ALCH> for Alchemy {
    type Error = Error;

    fn try_from(raw: ALCH) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let bounds = OBND::read(&mut cursor)?.try_into()?;
        let full_name = match (FULL::read(&mut cursor), raw.localized) {
            (Ok(f), true) => Some(LocalizedString::Localized(f.try_into()?)),
            (Ok(z), false) => Some(LocalizedString::ZString(z.try_into()?)),
            (Err(_), _) => None,
        };

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
        let model_filename = MODL::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let model_textures = MODT::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?
            .unwrap_or_else(Vec::new);
        let alternate_textures = MODS::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let icon = ICON::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let message_icon = MICO::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let pickup_sound = YNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let drop_sound = ZNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let weight = DATA::read(&mut cursor)?.try_into()?;
        let item = ENIT::read(&mut cursor)?.try_into()?;

        let mut effects = Vec::new();
        while let Ok(efid) = EFID::read(&mut cursor) {
            let efit = EFIT::read(&mut cursor)?.try_into()?;
            let mut ctdas = Vec::new();

            while let Ok(ctda) = CTDA::read(&mut cursor) {
                ctdas.push(ctda.try_into()?);
            }

            effects.push((efid.try_into()?, efit, ctdas));
        }

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            bounds,
            full_name,
            keywords,
            model_filename,
            model_textures,
            alternate_textures,
            icon,
            message_icon,
            pickup_sound,
            drop_sound,
            weight,
            item,
            effects,
        })
    }
}
