use std::io::Cursor;

use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};

use crate::{common::check_done_reading, error::Error};

#[binrw]
#[brw(little, magic = b"DMDT")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMDT {
    pub size: u16,
    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<DMDT> for Vec<[u8; 12]> {
    type Error = Error;

    fn try_from(raw: DMDT) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let mut result = Vec::new();
        while let Ok(d) = <[u8; 12]>::read_le(&mut cursor) {
            result.push(d)
        }
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
