use crate::common::{check_done_reading, FormID, WString32};
use crate::error::Error;
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"MODS")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MODS {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AlternateTexture {
    pub object_name: WString32,
    pub texture_set: FormID,
    pub threed_index: u32,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AlternateTextures {
    pub count: u32,

    #[br(count = count)]
    pub textures: Vec<AlternateTexture>,
}

impl TryInto<AlternateTextures> for MODS {
    type Error = Error;

    fn try_into(self) -> Result<AlternateTextures, Error> {
        Ok(AlternateTextures::read(&mut Cursor::new(&self.data))?)
    }
}

impl TryFrom<MODS> for Vec<AlternateTexture> {
    type Error = Error;

    fn try_from(raw: MODS) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let mut parsed = Vec::new();
        while let Ok(result) = AlternateTexture::read_le(&mut cursor) {
            parsed.push(result);
        }
        check_done_reading(&mut cursor)?;
        Ok(parsed)
    }
}
