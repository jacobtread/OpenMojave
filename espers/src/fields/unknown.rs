use binrw::binrw;
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UNKNOWN {
    pub kind: [u8; 4],
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}
