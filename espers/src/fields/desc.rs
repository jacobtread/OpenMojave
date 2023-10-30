use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead, NullString};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"DESC")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DESC {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryInto<String> for DESC {
    type Error = Error;

    fn try_into(self) -> Result<String, Self::Error> {
        Ok(NullString::read_le(&mut Cursor::new(&self.data))?.to_string())
    }
}

impl TryInto<u32> for DESC {
    type Error = Error;

    fn try_into(self) -> Result<u32, Self::Error> {
        let mut cursor = Cursor::new(&self.data);
        Ok(u32::read_le(&mut cursor)?)
    }
}
