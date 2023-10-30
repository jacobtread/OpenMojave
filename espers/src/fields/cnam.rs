use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead, BinWrite, NullString};
use rgb::RGBA8;
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"CNAM")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CNAM {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<CNAM> for RGBA8 {
    type Error = Error;

    fn try_from(raw: CNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result: [u8; 4] = BinRead::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result.into())
    }
}

impl TryFrom<CNAM> for u32 {
    type Error = Error;

    fn try_from(raw: CNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

impl TryFrom<CNAM> for String {
    type Error = Error;

    fn try_from(raw: CNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = NullString::read_le(&mut cursor)?.to_string();
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

impl TryFrom<String> for CNAM {
    type Error = Error;

    fn try_from(obj: String) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(Vec::new());
        NullString::from(obj).write(&mut cursor)?;
        let data = cursor.into_inner();

        Ok(Self {
            size: data.len() as u16,
            data,
        })
    }
}

impl TryFrom<CNAM> for Vec<FormID> {
    type Error = Error;

    fn try_from(raw: CNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let mut result = Vec::new();
        while let Ok(fid) = FormID::read_le(&mut cursor) {
            result.push(fid);
        }
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
