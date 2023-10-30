use super::{get_cursor, Flags, RecordHeader};
use crate::error::Error;
use crate::fields::EDID;
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"COBJ")]
pub struct COBJ {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructibleObj {
    pub header: RecordHeader,
    pub edid: String,
}

impl fmt::Display for ConstructibleObj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ConstructibleObj ({})", self.edid)
    }
}

impl TryFrom<COBJ> for ConstructibleObj {
    type Error = Error;

    fn try_from(raw: COBJ) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;

        Ok(Self {
            header: raw.header,
            edid,
        })
    }
}
