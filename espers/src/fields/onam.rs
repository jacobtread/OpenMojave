use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use binrw::{binrw, helpers::until_eof, BinRead, BinWrite, Endian, NullString};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"ONAM")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ONAM {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryInto<Vec<FormID>> for ONAM {
    type Error = Error;

    fn try_into(self) -> Result<Vec<FormID>, Error> {
        let mut cursor = Cursor::new(&self.data);
        Ok(until_eof(&mut cursor, Endian::Little, ())?)
    }
}

impl TryFrom<Vec<FormID>> for ONAM {
    type Error = Error;

    fn try_from(obj: Vec<FormID>) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(Vec::new());
        for fid in obj {
            fid.0.write_le(&mut cursor)?;
        }
        let data = cursor.into_inner();

        Ok(Self {
            size: data.len() as u16,
            data,
        })
    }
}

impl TryFrom<ONAM> for String {
    type Error = Error;

    fn try_from(raw: ONAM) -> Result<String, Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = NullString::read_le(&mut cursor)?.to_string();
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

impl TryFrom<ONAM> for u32 {
    type Error = Error;

    fn try_from(raw: ONAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
