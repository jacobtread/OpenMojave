use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"KSIZ")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KSIZ {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryInto<u32> for KSIZ {
    type Error = Error;

    fn try_into(self) -> Result<u32, Error> {
        Ok(u32::read_le(&mut Cursor::new(&self.data))?)
    }
}
