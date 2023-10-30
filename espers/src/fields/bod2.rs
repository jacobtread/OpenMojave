use crate::{common::check_done_reading, error::Error};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"BOD2")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BOD2 {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BodyTemplate2 {
    pub body_part_node_flags: u32,
    pub skill: u32,
}

impl TryFrom<BOD2> for BodyTemplate2 {
    type Error = Error;

    fn try_from(raw: BOD2) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
