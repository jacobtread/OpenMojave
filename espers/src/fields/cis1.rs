use crate::common::check_done_reading;
use crate::error::Error;
use binrw::{binrw, BinRead, NullString};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"CIS1")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CIS1 {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<CIS1> for String {
    type Error = Error;

    fn try_from(raw: CIS1) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = NullString::read(&mut cursor)?.to_string();
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
