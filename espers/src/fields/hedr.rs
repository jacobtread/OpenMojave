use binrw::binrw;
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"HEDR")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HEDR {
    pub size: u16,
    pub version: f32,
    pub records_and_groups_count: u32,
    pub next_available_object_id: u32,
}
