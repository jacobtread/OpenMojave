use crate::{common::check_done_reading, error::Error};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"ENAM")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ENAM {
    pub size: u16,
    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<ENAM> for u32 {
    type Error = Error;

    fn try_from(raw: ENAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
