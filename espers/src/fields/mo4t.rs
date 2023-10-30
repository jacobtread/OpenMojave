use super::MODT;
use binrw::binrw;
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"MO4T")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MO4T {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl From<MO4T> for MODT {
    fn from(raw: MO4T) -> Self {
        Self {
            size: raw.size,
            data: raw.data,
        }
    }
}
