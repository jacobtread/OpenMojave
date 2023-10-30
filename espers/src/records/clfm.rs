use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, LocalizedString};
use crate::error::Error;
use crate::fields::{CNAM, EDID, FNAM, FULL};
use crate::string_table::StringTables;
use binrw::{binrw, BinRead};
use rgb::RGBA8;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"CLFM")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CLFM {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,

    #[br(calc(localized))]
    #[bw(ignore)]
    pub localized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    pub header: RecordHeader,
    pub edid: String,
    pub full_name: Option<LocalizedString>,
    pub color: RGBA8,
    pub playable: u32,
}

impl Color {
    pub fn localize(&mut self, string_table: &StringTables) {
        if let Some(LocalizedString::Localized(l)) = self.full_name {
            if let Some(s) = string_table.get_string(&l) {
                self.full_name = Some(LocalizedString::ZString(s.clone()));
            }
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color ({})", self.edid)
    }
}

impl TryFrom<CLFM> for Color {
    type Error = Error;

    fn try_from(raw: CLFM) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let full_name = match (FULL::read(&mut cursor), raw.localized) {
            (Ok(f), true) => Some(LocalizedString::Localized(f.try_into()?)),
            (Ok(z), false) => Some(LocalizedString::ZString(z.try_into()?)),
            (Err(_), _) => None,
        };
        let color = CNAM::read(&mut cursor)?.try_into()?;
        let playable = FNAM::read(&mut cursor)?.try_into()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            full_name,
            color,
            playable,
        })
    }
}
