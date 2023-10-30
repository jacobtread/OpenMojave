use crate::common::check_done_reading;
use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead, NullString};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"GNAM")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GNAM {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<GNAM> for u32 {
    type Error = Error;

    fn try_from(raw: GNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

impl TryFrom<GNAM> for String {
    type Error = Error;

    fn try_from(raw: GNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = NullString::read_le(&mut cursor)?.to_string();
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
