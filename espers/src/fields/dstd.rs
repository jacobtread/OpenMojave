use crate::{
    common::{check_done_reading, FormID},
    error::Error,
};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"DSTD")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DSTD {
    pub size: u16,
    #[br(count = size)]
    pub data: Vec<u8>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageDataHeader {
    pub health_percent: u8,
    pub index: u8,
    pub damage_stage: u8,
    pub flags: u8,
    pub self_damage_rate: u32,
    pub explosion_id: FormID,
    pub debris_id: FormID,
    pub debris_count: u32,
}

impl TryFrom<DSTD> for StageDataHeader {
    type Error = Error;

    fn try_from(raw: DSTD) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
