use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead, NullString};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"TX01")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TX01 {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryInto<String> for TX01 {
    type Error = Error;

    fn try_into(self) -> Result<String, Error> {
        Ok(NullString::read_le(&mut Cursor::new(&self.data))?.to_string())
    }
}
