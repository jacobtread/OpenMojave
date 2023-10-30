use crate::error::Error;
use binrw::{binrw, BinRead, BinWrite};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"INCC")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct INCC {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryInto<u32> for INCC {
    type Error = Error;

    fn try_into(self) -> Result<u32, Self::Error> {
        Ok(u32::read_le(&mut Cursor::new(&self.data))?)
    }
}

impl TryFrom<u32> for INCC {
    type Error = Error;

    fn try_from(obj: u32) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(Vec::new());
        obj.write_le(&mut cursor)?;
        let data = cursor.into_inner();

        Ok(Self {
            size: data.len() as u16,
            data,
        })
    }
}
