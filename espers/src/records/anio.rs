use super::{get_cursor, Flags, RecordHeader};
use crate::common::check_done_reading;
use crate::error::Error;
use crate::fields::{AlternateTextures, Textures, BNAM, EDID, MODL, MODS, MODT};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"ANIO")]
pub struct ANIO {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimatedObjectInfo {
    pub header: RecordHeader,
    pub edid: String,
    pub model_filename: Option<String>,
    pub model_textures: Option<Textures>,
    pub alternate_textures: Option<AlternateTextures>,
    pub unload_event: Option<String>,
}

impl fmt::Display for AnimatedObjectInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AnimatedObjectInfo ({})", self.edid)
    }
}

impl TryFrom<ANIO> for AnimatedObjectInfo {
    type Error = Error;

    fn try_from(raw: ANIO) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let model_filename = MODL::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let model_textures = MODT::read(&mut cursor)
            .ok()
            .map(|modt| Textures::load(modt, raw.header.internal_version))
            .transpose()?;
        let alternate_textures = MODS::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let unload_event = BNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            model_filename,
            model_textures,
            alternate_textures,
            unload_event,
        })
    }
}
