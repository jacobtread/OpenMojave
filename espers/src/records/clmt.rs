use super::{get_cursor, Flags, RecordHeader};
use crate::common::check_done_reading;
use crate::error::Error;
use crate::fields::{ModelTextures, Weather, EDID, FNAM, GNAM, MODL, MODT, TNAM, WLST};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"CLMT")]
pub struct CLMT {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SunAndMoons {
    pub sunrise_begin: u8,
    pub sunrise_end: u8,
    pub sunset_begin: u8,
    pub sunset_end: u8,
    pub volatility: u8,
    pub moons: u8,
}

impl TryFrom<TNAM> for SunAndMoons {
    type Error = Error;

    fn try_from(raw: TNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Climate {
    pub header: RecordHeader,
    pub edid: String,
    pub weather_list: Vec<Weather>,
    pub sun_texture: Option<String>,
    pub glare_texture: Option<String>,
    pub model_filename: String,
    pub model_textures: Option<ModelTextures>,
    pub sun_and_moons: SunAndMoons,
}

impl fmt::Display for Climate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Climate ({})", self.edid)
    }
}

impl TryFrom<CLMT> for Climate {
    type Error = Error;

    fn try_from(raw: CLMT) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let weather_list = WLST::read(&mut cursor)?.try_into()?;
        let sun_texture = FNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let glare_texture = GNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let model_filename = MODL::read(&mut cursor)?.try_into()?;
        let model_textures = MODT::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let sun_and_moons = TNAM::read(&mut cursor)?.try_into()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            weather_list,
            sun_texture,
            glare_texture,
            model_filename,
            model_textures,
            sun_and_moons,
        })
    }
}
