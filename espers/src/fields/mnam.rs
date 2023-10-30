use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead, NullString};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"MNAM")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MNAM {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<MNAM> for String {
    type Error = Error;

    fn try_from(raw: MNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = NullString::read_le(&mut cursor)?.to_string();
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

impl TryFrom<MNAM> for FormID {
    type Error = Error;

    fn try_from(raw: MNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
