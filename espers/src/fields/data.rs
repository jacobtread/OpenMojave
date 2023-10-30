use crate::common::check_done_reading;
use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead, BinWrite};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"DATA")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DATA {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<Vec<u8>> for DATA {
    type Error = Error;

    fn try_from(obj: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self {
            size: obj.len() as u16,
            data: obj,
        })
    }
}

impl TryFrom<DATA> for Vec<u8> {
    type Error = Error;

    fn try_from(raw: DATA) -> Result<Self, Self::Error> {
        Ok(raw.data)
    }
}

impl TryFrom<DATA> for u32 {
    type Error = Error;

    fn try_from(raw: DATA) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

impl TryFrom<u32> for DATA {
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

impl TryFrom<DATA> for f32 {
    type Error = Error;

    fn try_from(raw: DATA) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

impl TryFrom<f32> for DATA {
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
