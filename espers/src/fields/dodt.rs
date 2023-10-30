use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead};
use bitflags::bitflags;
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"DODT")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DODT {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

bitflags! {
    #[binrw]
    #[derive(Deserialize, Serialize)]
    pub struct DODTFlags: u8 {
        const PARALLAX = 0x01;
        const ALPHA_BLENDING = 0x02;
        const ALPHA_TESTING = 0x04;
        const NOT_4_SUBTEXTURES = 0x08;
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DecalData {
    pub min_width: f32,
    pub max_width: f32,
    pub min_height: f32,
    pub max_height: f32,
    pub depth: f32,
    pub shininess: f32,
    pub parallax_scale: f32,
    pub parallax_passes: u8,
    pub flags: DODTFlags,
    pub unknown: [u8; 2],
    pub color: [u8; 4],
}

impl TryInto<DecalData> for DODT {
    type Error = Error;

    fn try_into(self) -> Result<DecalData, Error> {
        Ok(DecalData::read(&mut Cursor::new(&self.data))?)
    }
}
