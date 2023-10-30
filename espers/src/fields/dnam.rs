use crate::{common::check_done_reading, error::Error};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"DNAM")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DNAM {
    pub size: u16,
    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<DNAM> for u8 {
    type Error = Error;

    fn try_from(raw: DNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

impl TryFrom<DNAM> for u16 {
    type Error = Error;

    fn try_from(raw: DNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

impl TryFrom<DNAM> for u32 {
    type Error = Error;

    fn try_from(raw: DNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
