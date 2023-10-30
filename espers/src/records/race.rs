use super::{get_cursor, Flags, RecordHeader};
use crate::error::Error;
use crate::fields::{DESC, EDID, FULL};
use binrw::binrw;
use binrw::BinRead;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"RACE")]
pub struct RACE {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Race {
    pub header: RecordHeader,
    pub edid: String,
    pub full_name: Option<String>,
    pub description: String,
}

impl fmt::Display for Race {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Race ({})", self.edid)
    }
}

impl TryFrom<RACE> for Race {
    type Error = Error;

    fn try_from(raw: RACE) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let full_name = FULL::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let description = DESC::read(&mut cursor)?.try_into()?;

        Ok(Self {
            header: raw.header,
            edid,
            full_name,
            description,
        })
    }
}
