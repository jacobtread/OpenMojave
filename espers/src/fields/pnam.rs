use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use binrw::{binrw, BinRead};
use rgb::RGBA8;
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"PNAM")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PNAM {
    pub size: u16,
    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<PNAM> for u32 {
    type Error = Error;

    fn try_from(raw: PNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

impl TryFrom<PNAM> for f32 {
    type Error = Error;

    fn try_from(raw: PNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

impl TryFrom<PNAM> for RGBA8 {
    type Error = Error;

    fn try_from(raw: PNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result: [u8; 4] = BinRead::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result.into())
    }
}

impl TryFrom<PNAM> for Vec<FormID> {
    type Error = Error;

    fn try_from(raw: PNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(raw.data);
        let mut result = Vec::new();
        while let Ok(fid) = FormID::read(&mut cursor) {
            result.push(fid);
        }
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
