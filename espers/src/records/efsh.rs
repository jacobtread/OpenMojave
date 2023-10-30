use super::{get_cursor, Flags, RecordHeader};
use crate::common::check_done_reading;
use crate::error::Error;
use crate::fields::{DATA, EDID, ICO2, ICON, NAM7, NAM8, NAM9};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"EFSH")]
pub struct EFSH {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectShader {
    pub header: RecordHeader,
    pub edid: String,
    pub start_effect: String,
    pub looped_effect: String,
    pub post_effect: String,
    pub looped_gradient: Option<String>,
    pub end_gradient: Option<String>,
    pub shader_data: Vec<u8>,
}

impl fmt::Display for EffectShader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EffectShader ({})", self.edid)
    }
}

impl TryFrom<EFSH> for EffectShader {
    type Error = Error;

    fn try_from(raw: EFSH) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let start_effect = ICON::read(&mut cursor)?.try_into()?;
        let looped_effect = ICO2::read(&mut cursor)?.try_into()?;
        let post_effect = NAM7::read(&mut cursor)?.try_into()?;
        let looped_gradient = NAM8::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let end_gradient = NAM9::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let shader_data = DATA::read(&mut cursor)?.try_into()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            start_effect,
            looped_effect,
            post_effect,
            looped_gradient,
            end_gradient,
            shader_data,
        })
    }
}
