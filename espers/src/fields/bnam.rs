use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead, NullString};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"BNAM")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BNAM {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<BNAM> for String {
    type Error = Error;

    fn try_from(raw: BNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = NullString::read_le(&mut cursor)?.to_string();
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

impl TryFrom<BNAM> for FormID {
    type Error = Error;

    fn try_from(raw: BNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

impl TryFrom<BNAM> for u32 {
    type Error = Error;

    fn try_from(raw: BNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
