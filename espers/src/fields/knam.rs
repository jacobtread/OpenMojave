use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"KNAM")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KNAM {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryInto<FormID> for KNAM {
    type Error = Error;

    fn try_into(self) -> Result<FormID, Self::Error> {
        let mut cursor = Cursor::new(&self.data);
        let parsed = FormID::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(parsed)
    }
}
