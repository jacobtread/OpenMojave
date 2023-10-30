use super::MODS;
use binrw::binrw;
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"MO2S")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MO2S {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl From<MO2S> for MODS {
    fn from(raw: MO2S) -> Self {
        Self {
            size: raw.size,
            data: raw.data,
        }
    }
}
