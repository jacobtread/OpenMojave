use super::{AlternateTextures, ModelTextures, Unknown4, MODS, MODT};
use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead, NullString};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"MODL")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MODL {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<MODL> for String {
    type Error = Error;

    fn try_from(raw: MODL) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = NullString::read_le(&mut cursor)?.to_string();
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

impl TryFrom<MODL> for FormID {
    type Error = Error;

    fn try_from(raw: MODL) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Textures {
    Header(ModelTextures),
    NoHeader(Vec<Unknown4>),
}

impl Textures {
    pub fn load(modt: MODT, version: u16) -> Result<Self, Error> {
        Ok(match version {
            0..=37 => Err(Error::UnknownVersion(format!("{:?}", modt), version))?,
            38..=39 => Self::NoHeader(modt.try_into()?),
            40.. => Self::Header(modt.try_into()?),
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Model {
    pub model: String,
    pub textures: Option<Textures>,
    pub alternate_textures: Option<AlternateTextures>,
}

impl Model {
    pub fn load(cursor: &mut Cursor<&Vec<u8>>, version: u16) -> Result<Self, Error> {
        let model = MODL::read(cursor)?.try_into()?;
        let textures = MODT::read(cursor)
            .ok()
            .map(|m| Textures::load(m, version))
            .transpose()?;
        let alternate_textures = MODS::read(cursor).ok().map(TryInto::try_into).transpose()?;

        Ok(Self {
            model,
            textures,
            alternate_textures,
        })
    }

    pub fn try_load(cursor: &mut Cursor<&Vec<u8>>, version: u16) -> Result<Option<Self>, Error> {
        let model = match MODL::read(cursor) {
            Ok(m) => m.try_into()?,
            Err(_) => return Ok(None),
        };
        let textures = MODT::read(cursor)
            .ok()
            .map(|m| Textures::load(m, version))
            .transpose()?;
        let alternate_textures = MODS::read(cursor).ok().map(TryInto::try_into).transpose()?;

        Ok(Some(Self {
            model,
            textures,
            alternate_textures,
        }))
    }
}
