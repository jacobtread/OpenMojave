use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead, BinWrite, NullString};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"WNAM")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WNAM {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryInto<String> for WNAM {
    type Error = Error;

    fn try_into(self) -> Result<String, Self::Error> {
        Ok(NullString::read_le(&mut Cursor::new(&self.data))?.to_string())
    }
}

impl TryFrom<String> for WNAM {
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

impl TryInto<FormID> for WNAM {
    type Error = Error;

    fn try_into(self) -> Result<FormID, Self::Error> {
        let mut cursor = Cursor::new(&self.data);
        let parsed = FormID::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(parsed)
    }
}

impl TryFrom<FormID> for WNAM {
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
