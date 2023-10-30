use super::{
    dmdl::DMDL,
    dmds::{DestructionTextures, DMDS},
    dmdt::DMDT,
    dstd::{StageDataHeader, DSTD},
    dstf::DSTF,
};
use crate::{common::check_done_reading, error::Error};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"DEST")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DEST {
    pub size: u16,
    #[br(count = size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageData {
    pub header: StageDataHeader,
    pub replacement_model: Option<String>,
    pub unknown1: Vec<[u8; 12]>,
    pub destruction_textures: Option<DestructionTextures>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestructionData {
    pub data: DestructionDataHeader,
    pub stage_data: Vec<StageData>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestructionDataHeader {
    pub health: u32,
    pub count: u8,
    pub flag: u8,
    pub unknown1: u8,
    pub unknown2: u8,
}

impl TryFrom<DEST> for DestructionDataHeader {
    type Error = Error;

    fn try_from(raw: DEST) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

impl DestructionData {
    pub fn load(cursor: &mut Cursor<&Vec<u8>>) -> Result<Option<Self>, Error> {
        let data: DestructionDataHeader = match DEST::read(cursor) {
            Ok(d) => d.try_into()?,
            Err(_) => return Ok(None),
        };

        let mut stage_data = Vec::new();
        for _ in 0..data.count {
            let header = DSTD::read(cursor)?.try_into()?;
            let replacement_model = DMDL::read(cursor).ok().map(TryInto::try_into).transpose()?;
            let unknown1 = DMDT::read(cursor)
                .ok()
                .map(TryInto::try_into)
                .transpose()?
                .unwrap_or_else(Vec::new);
            let destruction_textures =
                DMDS::read(cursor).ok().map(TryInto::try_into).transpose()?;
            DSTF::read(cursor)?;

            stage_data.push(StageData {
                header,
                replacement_model,
                unknown1,
                destruction_textures,
            })
        }

        Ok(Some(Self { data, stage_data }))
    }
}
