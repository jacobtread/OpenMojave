use crate::common::check_done_reading;
use crate::error::Error;
use binrw::binrw;
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"XPPA")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct XPPA {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<XPPA> for () {
    type Error = Error;

    fn try_from(raw: XPPA) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        check_done_reading(&mut cursor)?;
        Ok(())
    }
}

impl TryFrom<()> for XPPA {
    type Error = Error;

    fn try_from(_: ()) -> Result<Self, Self::Error> {
        Ok(Self {
            size: 0,
            data: Vec::new(),
        })
    }
}
