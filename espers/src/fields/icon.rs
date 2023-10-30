use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead, NullString};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"ICON")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ICON {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryInto<String> for ICON {
    type Error = Error;

    fn try_into(self) -> Result<String, Error> {
        Ok(NullString::read_le(&mut Cursor::new(&self.data))?.to_string())
    }
}
