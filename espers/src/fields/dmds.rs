use crate::common::{check_done_reading, FormID, WString32};
use crate::error::Error;
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"DMDS")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMDS {
    pub size: u16,
    #[br(count = size)]
    pub data: Vec<u8>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestructionTexture {
    pub name: WString32,
    pub texture_id: FormID,
    pub unknown1: u32,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestructionTextures {
    pub count: u32,

    #[br(count = count)]
    pub textures: Vec<DestructionTexture>,
}

impl TryFrom<DMDS> for DestructionTextures {
    type Error = Error;

    fn try_from(raw: DMDS) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
