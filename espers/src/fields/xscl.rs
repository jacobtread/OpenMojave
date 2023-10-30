use crate::common::check_done_reading;
use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead, BinWrite};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"XSCL")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct XSCL {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<XSCL> for f32 {
    type Error = Error;

    fn try_from(raw: XSCL) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = f32::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

impl TryFrom<f32> for XSCL {
    type Error = Error;

    fn try_from(obj: f32) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(Vec::new());
        obj.write_le(&mut cursor)?;
        let data = cursor.into_inner();

        Ok(Self {
            size: data.len() as u16,
            data,
        })
    }
}
