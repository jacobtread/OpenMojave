use crate::{common::check_done_reading, error::Error};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"BODT")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BODT {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BodyTemplate {
    pub body_part_node_flags: u32,
    pub flags: Option<u8>,
    pub junk_data: Option<[u8; 3]>,
    pub skill: u32,
}

impl BodyTemplate {
    pub fn load(raw: BODT, version: u16) -> Result<Self, Error> {
        let mut cursor = Cursor::new(&raw.data);
        Ok(match version {
            0..=21 => {
                let body_part_node_flags = u32::read_le(&mut cursor)?;
                let flags = None;
                let junk_data = None;
                let skill = u32::read_le(&mut cursor)?;
                check_done_reading(&mut cursor)?;
                Self {
                    body_part_node_flags,
                    flags,
                    junk_data,
                    skill,
                }
            }
            22.. => {
                let body_part_node_flags = u32::read_le(&mut cursor)?;
                let flags = Some(u8::read(&mut cursor)?);
                let junk_data = Some(BinRead::read(&mut cursor)?);
                let skill = u32::read_le(&mut cursor)?;
                check_done_reading(&mut cursor)?;
                Self {
                    body_part_node_flags,
                    flags,
                    junk_data,
                    skill,
                }
            }
        })
    }
}
