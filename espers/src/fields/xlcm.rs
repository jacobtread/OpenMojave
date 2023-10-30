use crate::common::check_done_reading;
use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead, BinWrite};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"XLCM")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct XLCM {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<XLCM> for u32 {
    type Error = Error;

    fn try_from(raw: XLCM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = u32::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

impl TryFrom<u32> for XLCM {
    type Error = Error;

    fn try_from(obj: u32) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(Vec::new());
        obj.write_le(&mut cursor)?;
        let data = cursor.into_inner();

        Ok(Self {
            size: data.len() as u16,
            data,
        })
    }
}
