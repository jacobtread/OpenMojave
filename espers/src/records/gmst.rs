use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, LocalizedString};
use crate::error::Error;
use crate::fields::{DATA, EDID};
use crate::string_table::StringTables;
use binrw::{binrw, BinRead, NullString};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"GMST")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GMST {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,

    #[br(calc(localized))]
    #[bw(ignore)]
    pub localized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Bool(u32),
    Int(u32),
    Float(f32),
    Str(LocalizedString),
    Unknown([u8; 4]),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSetting {
    pub header: RecordHeader,
    pub edid: String,
    pub value: Value,
}

impl GameSetting {
    pub fn localize(&mut self, string_table: &StringTables) {
        if let Value::Str(LocalizedString::Localized(l)) = self.value {
            if let Some(s) = string_table.get_string(&l) {
                self.value = Value::Str(LocalizedString::ZString(s.clone()));
            }
        }
    }
}

impl fmt::Display for GameSetting {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GameSetting ({})", self.edid)
    }
}

impl TryFrom<GMST> for GameSetting {
    type Error = Error;

    fn try_from(raw: GMST) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid: String = EDID::read(&mut cursor)?.try_into()?;
        let data = DATA::read(&mut cursor)?;
        let mut data_cursor = Cursor::new(&data.data);
        let value = match &edid.chars().next() {
            Some('b') => Value::Bool(u32::read_le(&mut data_cursor)?),
            Some('i') => Value::Int(u32::read_le(&mut data_cursor)?),
            Some('f') => Value::Float(f32::read_le(&mut data_cursor)?),
            Some('s') => Value::Str(if raw.localized {
                LocalizedString::Localized(u32::read_le(&mut data_cursor)?)
            } else {
                LocalizedString::ZString(NullString::read(&mut data_cursor)?.to_string())
            }),
            _ => Value::Unknown(BinRead::read_le(&mut data_cursor)?),
        };

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            value,
        })
    }
}
