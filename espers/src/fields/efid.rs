use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

use super::{EffectCondition, EffectItem, EFIT};

#[binrw]
#[brw(little, magic = b"EFID")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EFID {
    pub size: u16,
    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<EFID> for FormID {
    type Error = Error;

    fn try_from(raw: EFID) -> Result<FormID, Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Effect {
    pub id: FormID,
    pub item: EffectItem,
    pub conditions: Vec<EffectCondition>,
}

impl Effect {
    pub fn load(cursor: &mut Cursor<&Vec<u8>>) -> Result<Self, Error> {
        let id = EFID::read(cursor)?.try_into()?;
        let item = EFIT::read(cursor)?.try_into()?;
        let mut conditions = Vec::new();

        while let Ok(ec) = EffectCondition::load(cursor) {
            conditions.push(ec);
        }

        Ok(Self {
            id,
            item,
            conditions,
        })
    }
}
