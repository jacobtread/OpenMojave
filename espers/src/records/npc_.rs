use super::{get_cursor, Flags, RecordHeader};
use crate::error::Error;
use crate::fields::EDID;
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"NPC_")]
pub struct NPC_ {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPC {
    pub header: RecordHeader,
    pub edid: String,
}

impl fmt::Display for NPC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NPC ({})", self.edid)
    }
}

impl TryFrom<NPC_> for NPC {
    type Error = Error;

    fn try_from(raw: NPC_) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;

        Ok(Self {
            header: raw.header,
            edid,
        })
    }
}
