use crate::{common::check_done_reading, error::Error};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"EAMT")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EAMT {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<EAMT> for u16 {
    type Error = Error;

    fn try_from(raw: EAMT) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
