use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead, BinWrite};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"XOWN")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct XOWN {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<XOWN> for FormID {
    type Error = Error;

    fn try_from(raw: XOWN) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = FormID::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

impl TryFrom<FormID> for XOWN {
    type Error = Error;

    fn try_from(obj: FormID) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(Vec::new());
        obj.write(&mut cursor)?;
        let data = cursor.into_inner();

        Ok(Self {
            size: data.len() as u16,
            data,
        })
    }
}
