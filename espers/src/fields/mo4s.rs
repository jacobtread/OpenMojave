use super::MODS;
use binrw::binrw;
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"MO4S")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MO4S {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl From<MO4S> for MODS {
    fn from(raw: MO4S) -> Self {
        Self {
            size: raw.size,
            data: raw.data,
        }
    }
}
