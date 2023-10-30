use binrw::binrw;
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"DSTF")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DSTF {
    pub size: u16,
    #[br(count = size)]
    pub data: Vec<u8>,
}
