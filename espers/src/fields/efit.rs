use crate::common::check_done_reading;
use crate::error::Error;
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"EFIT")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EFIT {
    pub size: u16,
    #[br(count = size)]
    pub data: Vec<u8>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectItem {
    pub magnitude: f32,
    pub u1: u32,
    pub duration: u32,
    pub u2: u32,
    pub actor_value_code: u32,
}

impl TryFrom<EFIT> for EffectItem {
    type Error = Error;

    fn try_from(raw: EFIT) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
