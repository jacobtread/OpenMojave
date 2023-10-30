use super::{get_cursor, Flags, RecordHeader};
use crate::error::Error;
use crate::fields::{CNAM, EDID};
use binrw::{binrw, BinRead};
use rgb::RGBA8;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"LCRT")]
pub struct LCRT {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationRef {
    pub header: RecordHeader,
    pub edid: String,
    pub color: Option<RGBA8>,
}

impl fmt::Display for LocationRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LocationRef ({})", self.edid)
    }
}

impl TryFrom<LCRT> for LocationRef {
    type Error = Error;

    fn try_from(raw: LCRT) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let color = CNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        Ok(Self {
            header: raw.header,
            edid,
            color,
        })
    }
}
