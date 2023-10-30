use crate::common::FormID;
use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"ZNAM")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ZNAM {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryInto<FormID> for ZNAM {
    type Error = Error;

    fn try_into(self) -> Result<FormID, Error> {
        Ok(FormID::read_le(&mut Cursor::new(&self.data))?)
    }
}
