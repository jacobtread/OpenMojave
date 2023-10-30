use crate::common::check_done_reading;
use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead, BinWrite};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"XAPD")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct XAPD {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<XAPD> for u8 {
    type Error = Error;

    fn try_from(raw: XAPD) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = u8::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

impl TryFrom<u8> for XAPD {
    type Error = Error;

    fn try_from(obj: u8) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(Vec::new());
        obj.write(&mut cursor)?;
        let data = cursor.into_inner();

        Ok(Self {
            size: data.len() as u16,
            data,
        })
    }
}
