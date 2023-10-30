use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, LocalizedString};
use crate::error::Error;
use crate::fields::{ATTR, DATA, DESC, EDID, FULL, ICON};
use crate::string_table::StringTables;
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"CLAS")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CLAS {
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
pub struct ClassData {
    pub tag_skill_1: u32,
    pub tag_skill_2: u32,
    pub tag_skill_3: u32,
    // -1 indicates no more tag skills/may be updated if player uses Tag! Perk
    pub unused_tag_slot_1: u32,
    pub unused_tag_slot_2: u32,
    pub unused_tag_slot_3: u32,
    pub unused_tag_slot_4: u32,
}

impl TryFrom<DATA> for ClassData {
    type Error = Error;

    fn try_from(raw: DATA) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        Ok(Self::read(&mut cursor)?)
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassSpecialAttributes {
    pub strength: u8,
    pub perception: u8,
    pub endurance: u8,
    pub charisma: u8,
    pub intelligence: u8,
    pub agility: u8,
    pub luck: u8,
}

impl TryFrom<ATTR> for ClassSpecialAttributes {
    type Error = Error;

    fn try_from(raw: ATTR) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        Ok(Self::read(&mut cursor)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub header: RecordHeader,
    pub edid: String,
    pub full_name: LocalizedString,
    pub description: LocalizedString,
    pub icon: Option<String>,
    pub data: ClassData,
    pub class_special_attr: ClassSpecialAttributes,
}

impl Class {
    pub fn localize(&mut self, string_table: &StringTables) {
        if let LocalizedString::Localized(l) = self.full_name {
            if let Some(s) = string_table.get_string(&l) {
                self.full_name = LocalizedString::ZString(s.clone());
            }
        }

        if let LocalizedString::Localized(l) = self.description {
            if let Some(s) = string_table.get_string(&l) {
                self.description = LocalizedString::ZString(s.clone());
            }
        }
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Class ({})", self.edid)
    }
}

impl TryFrom<CLAS> for Class {
    type Error = Error;

    fn try_from(raw: CLAS) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let full_name = if raw.localized {
            LocalizedString::Localized(FULL::read(&mut cursor)?.try_into()?)
        } else {
            LocalizedString::ZString(FULL::read(&mut cursor)?.try_into()?)
        };
        let description = if raw.localized {
            LocalizedString::Localized(DESC::read(&mut cursor)?.try_into()?)
        } else {
            LocalizedString::ZString(DESC::read(&mut cursor)?.try_into()?)
        };
        let icon = ICON::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let data = DATA::read(&mut cursor)?.try_into()?;
        let class_special_attr = ATTR::read(&mut cursor)?.try_into()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            full_name,
            description,
            icon,
            data,
            class_special_attr,
        })
    }
}
