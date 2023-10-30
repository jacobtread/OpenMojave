use crate::common::FormID;
use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"PLCN")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PLCN {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryInto<FormID> for PLCN {
    type Error = Error;

    fn try_into(self) -> Result<FormID, Error> {
        Ok(FormID::read_le(&mut Cursor::new(&self.data))?)
    }
}
