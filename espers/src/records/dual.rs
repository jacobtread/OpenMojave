use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use crate::fields::{ObjectBounds, DATA, EDID, OBND};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"DUAL")]
pub struct DUAL {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualCastData {
    pub projectile: FormID,
    pub explosion: FormID,
    pub effect_shader: FormID,
    pub hit_effect_art: FormID,
    pub impact_data_set: FormID,
    pub scale_inheritance_flags: u32,
}

impl TryFrom<DATA> for DualCastData {
    type Error = Error;

    fn try_from(raw: DATA) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualCastArt {
    pub header: RecordHeader,
    pub edid: String,
    pub bounds: ObjectBounds,
    pub data: DualCastData,
}

impl fmt::Display for DualCastArt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DualCastArt ({})", self.edid)
    }
}

impl TryFrom<DUAL> for DualCastArt {
    type Error = Error;

    fn try_from(raw: DUAL) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let bounds = OBND::read(&mut cursor)?.try_into()?;
        let data = DATA::read(&mut cursor)?.try_into()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            bounds,
            data,
        })
    }
}
