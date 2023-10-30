use super::{get_cursor, Flags, RecordHeader};
use crate::common::check_done_reading;
use crate::error::Error;
use crate::fields::{CNAM, EDID};
use binrw::{binrw, BinRead, BinWrite};
use rgb::RGBA8;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little)]
pub struct UNKNOWN {
    pub magic: u32,

    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unknown {
    pub magic: String,
    pub header: RecordHeader,
    pub data: Vec<u8>,
}

impl fmt::Display for Unknown {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unknown ({})", self.magic)
    }
}

impl TryFrom<UNKNOWN> for Unknown {
    type Error = Error;

    fn try_from(raw: UNKNOWN) -> Result<Self, Self::Error> {
        let magic = String::from_utf8_lossy(&raw.magic.to_be_bytes()).to_string();

        Ok(Self {
            magic,
            header: raw.header,
            data: raw.data,
        })
    }
}

impl TryFrom<Unknown> for UNKNOWN {
    type Error = Error;

    fn try_from(obj: Unknown) -> Result<Self, Self::Error> {
        Ok(Self {
            magic: 0,
            header: obj.header,
            data: obj.data,
        })
    }
}
