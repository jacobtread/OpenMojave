use std::io::Cursor;

use binrw::{binrw, BinRead, BinReaderExt, NullString};

use crate::esp::{
    error::EspError,
    shared::{require_complete, FormId},
};

#[binrw]
#[brw(little, magic = b"CNAM")]
#[derive(Debug, Clone)]
pub struct CNAM {
    pub size: u16,
    #[br(count = size)]
    pub data: Vec<u8>,
}

impl CNAM {
    #[inline]
    fn cursor(&self) -> Cursor<&[u8]> {
        Cursor::new(&self.data)
    }
}

impl TryFrom<CNAM> for u32 {
    type Error = EspError;

    fn try_from(raw: CNAM) -> Result<Self, Self::Error> {
        let mut cursor = raw.cursor();
        let result: u32 = cursor.read_le()?;
        require_complete(&mut cursor)?;
        Ok(result)
    }
}

impl TryFrom<CNAM> for String {
    type Error = EspError;

    fn try_from(raw: CNAM) -> Result<Self, Self::Error> {
        let mut cursor = raw.cursor();
        let result = NullString::read_le(&mut cursor)?.to_string();
        require_complete(&mut cursor)?;
        Ok(result)
    }
}

impl TryFrom<CNAM> for Vec<FormId> {
    type Error = EspError;

    fn try_from(raw: CNAM) -> Result<Self, Self::Error> {
        let mut cursor = raw.cursor();
        let mut result = Vec::new();
        while let Ok(fid) = FormId::read_le(&mut cursor) {
            result.push(fid);
        }
        require_complete(&mut cursor)?;
        Ok(result)
    }
}
