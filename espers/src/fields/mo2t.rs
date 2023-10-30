use super::MODT;
use binrw::binrw;
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"MO2T")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MO2T {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl From<MO2T> for MODT {
    fn from(raw: MO2T) -> Self {
        Self {
            size: raw.size,
            data: raw.data,
        }
    }
}
