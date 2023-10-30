use crate::common::check_done_reading;
use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead, NullString};
use rgb::RGBA8;
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"FNAM")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FNAM {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<FNAM> for u8 {
    type Error = Error;

    fn try_from(raw: FNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

impl TryFrom<FNAM> for u16 {
    type Error = Error;

    fn try_from(raw: FNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

impl TryFrom<FNAM> for u32 {
    type Error = Error;

    fn try_from(raw: FNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

impl TryFrom<FNAM> for RGBA8 {
    type Error = Error;

    fn try_from(raw: FNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result: [u8; 4] = BinRead::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result.into())
    }
}

impl TryFrom<FNAM> for String {
    type Error = Error;

    fn try_from(raw: FNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = NullString::read_le(&mut cursor)?.to_string();
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
