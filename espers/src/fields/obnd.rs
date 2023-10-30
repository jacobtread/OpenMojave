use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"OBND")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OBND {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ObjectBounds {
    pub x1: i16,
    pub y1: i16,
    pub z1: i16,
    pub x2: i16,
    pub y2: i16,
    pub z2: i16,
}

impl TryInto<ObjectBounds> for OBND {
    type Error = Error;

    fn try_into(self) -> Result<ObjectBounds, Self::Error> {
        Ok(ObjectBounds::read(&mut Cursor::new(&self.data))?)
    }
}
