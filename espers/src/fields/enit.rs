use crate::common::check_done_reading;
use crate::error::Error;
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"ENIT")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ENIT {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnchantedItem {
    pub ty: u32,
    pub u1: u32,
    pub u2: u32,
    pub flags: u32,
}

impl TryFrom<ENIT> for EnchantedItem {
    type Error = Error;

    fn try_from(raw: ENIT) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
