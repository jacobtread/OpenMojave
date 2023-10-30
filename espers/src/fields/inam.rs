use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use binrw::{binrw, helpers::until_eof, io::Cursor, BinRead, Endian};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"INAM")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct INAM {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<INAM> for FormID {
    type Error = Error;

    fn try_from(raw: INAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

impl TryFrom<INAM> for Vec<FormID> {
    type Error = Error;

    fn try_from(raw: INAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = until_eof(&mut cursor, Endian::Little, ())?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
