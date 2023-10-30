use crate::common::check_done_reading;
use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"PFPC")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PFPC {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<PFPC> for [u8; 4] {
    type Error = Error;

    fn try_from(raw: PFPC) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
