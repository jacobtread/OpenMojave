use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, LocalizedString};
use crate::error::Error;
use crate::fields::{Effect, EnchantedItem, ObjectBounds, EDID, ENIT, FULL, OBND};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"ENCH")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ENCH {
    pub header: RecordHeader,
    #[br(count = header.size)]
    pub data: Vec<u8>,

    #[br(calc(localized))]
    #[bw(ignore)]
    pub localized: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enchantment {
    pub header: RecordHeader,
    pub edid: String,
    pub full_name: Option<LocalizedString>,
    pub item: EnchantedItem,
    pub effects: Vec<Effect>,
}

impl fmt::Display for Enchantment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Enchantment ({})", self.edid)
    }
}

impl TryFrom<ENCH> for Enchantment {
    type Error = Error;

    fn try_from(raw: ENCH) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let full_name = match (FULL::read(&mut cursor), raw.localized) {
            (Ok(f), true) => Some(LocalizedString::Localized(f.try_into()?)),
            (Ok(z), false) => Some(LocalizedString::ZString(z.try_into()?)),
            (Err(_), _) => None,
        };
        let item = ENIT::read(&mut cursor)?.try_into()?;

        let mut effects = Vec::new();

        while let Ok(e) = Effect::load(&mut cursor) {
            effects.push(e);
        }

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            full_name,
            item,
            effects,
        })
    }
}
