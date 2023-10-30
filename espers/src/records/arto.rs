use super::{get_cursor, Flags, RecordHeader};
use crate::common::check_done_reading;
use crate::error::Error;
use crate::fields::AlternateTextures;
use crate::fields::ObjectBounds;
use crate::fields::Textures;
use crate::fields::DNAM;
use crate::fields::EDID;
use crate::fields::MODL;
use crate::fields::MODS;
use crate::fields::MODT;
use crate::fields::OBND;
use binrw::binrw;
use binrw::BinRead;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"ARTO")]
pub struct ARTO {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtObject {
    pub header: RecordHeader,
    pub edid: String,
    pub bounds: ObjectBounds,
    pub model_filename: Option<String>,
    pub model_textures: Option<Textures>,
    pub alternate_textures: Option<AlternateTextures>,
    pub art_type: u32,
}

impl fmt::Display for ArtObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ArtObject ({})", self.edid)
    }
}

impl TryFrom<ARTO> for ArtObject {
    type Error = Error;

    fn try_from(raw: ARTO) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let bounds = OBND::read(&mut cursor)?.try_into()?;
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
        let art_type = DNAM::read(&mut cursor)?.try_into()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            bounds,
            model_filename,
            model_textures,
            alternate_textures,
            art_type,
        })
    }
}
