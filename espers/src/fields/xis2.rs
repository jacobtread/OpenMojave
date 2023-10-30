use binrw::binrw;
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"XIS2")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct XIS2 {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}
