use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"WLST")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WLST {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Weather {
    pub weather: FormID,
    pub chance: u32,
    pub global: FormID,
}

impl TryFrom<WLST> for Vec<Weather> {
    type Error = Error;

    fn try_from(raw: WLST) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let mut result = Vec::new();
        while let Ok(w) = Weather::read(&mut cursor) {
            result.push(w);
        }
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
