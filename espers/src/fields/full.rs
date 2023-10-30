use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead, NullString};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"FULL")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FULL {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryInto<String> for FULL {
    type Error = Error;

    fn try_into(self) -> Result<String, Error> {
        Ok(NullString::read_le(&mut Cursor::new(&self.data))?.to_string())
    }
}

impl TryInto<u32> for FULL {
    type Error = Error;

    fn try_into(self) -> Result<u32, Error> {
        let mut cursor = Cursor::new(&self.data);
        Ok(BinRead::read_le(&mut cursor)?)
    }
}
