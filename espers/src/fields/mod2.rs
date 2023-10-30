use super::MODL;
use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"MOD2")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MOD2 {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<MOD2> for FormID {
    type Error = Error;

    fn try_from(raw: MOD2) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

impl From<MOD2> for MODL {
    fn from(raw: MOD2) -> Self {
        Self {
            size: raw.size,
            data: raw.data,
        }
    }
}
