use binrw::binrw;
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"XRGB")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct XRGB {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}
