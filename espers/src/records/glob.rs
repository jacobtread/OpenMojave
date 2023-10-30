use super::{get_cursor, Flags, RecordHeader};
use crate::error::Error;
use crate::fields::{ObjectBounds, EDID, OBND};
use binrw::binrw;
use binrw::BinRead;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"GLOB")]
pub struct GLOB {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalVariable {
    pub header: RecordHeader,
    pub edid: String,
    pub obnd: Option<ObjectBounds>,
}

impl fmt::Display for GlobalVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Global Variable ({})", self.edid)
    }
}

impl TryFrom<GLOB> for GlobalVariable {
    type Error = Error;

    fn try_from(raw: GLOB) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let obnd = OBND::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        Ok(Self {
            header: raw.header,
            edid,
            obnd,
        })
    }
}
