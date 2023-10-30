use crate::{common::check_done_reading, error::Error};
use binrw::{binrw, BinRead, NullString};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"DMDL")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMDL {
    pub size: u16,
    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<DMDL> for String {
    type Error = Error;

    fn try_from(raw: DMDL) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = NullString::read_le(&mut cursor)?.to_string();
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
