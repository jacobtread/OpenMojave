use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead, BinWrite, NullString};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"EDID")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EDID {
    pub size: u16,
    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryInto<String> for EDID {
    type Error = Error;

    fn try_into(self) -> Result<String, Self::Error> {
        Ok(NullString::read_le(&mut Cursor::new(&self.data))?.to_string())
    }
}

impl TryFrom<String> for EDID {
    type Error = Error;

    fn try_from(obj: String) -> Result<EDID, Self::Error> {
        let mut cursor = Cursor::new(Vec::new());
        NullString::from(obj).write(&mut cursor)?;
        let data = cursor.into_inner();

        Ok(Self {
            size: data.len() as u16,
            data,
        })
    }
}
