use super::{get_cursor, Flags, RecordHeader};
use crate::error::Error;
use crate::fields::{
    DecalData, ObjectBounds, DNAM, DODT, EDID, OBND, TX00, TX01, TX02, TX03, TX04, TX05, TX06, TX07,
};
use binrw::{binrw, BinRead};
use bitflags::bitflags;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

bitflags! {
    #[binrw]
    #[brw(little)]
    #[derive(Deserialize, Serialize)]
    pub struct TypeFlags: u16 {
        const NO_SPECULAR_MAP = 0x01;
        const FACEGEN_TEXTURES = 0x02;
        const MODEL_SPACE_NORMAL_MAP = 0x04;
    }
}

impl TryFrom<DNAM> for TypeFlags {
    type Error = Error;

    fn try_from(raw: DNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        Ok(Self::read(&mut cursor)?)
    }
}

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"TXST")]
pub struct TXST {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextureSet {
    pub header: RecordHeader,
    pub edid: Option<String>,
    pub obnd: Option<ObjectBounds>,
    pub color_map: Option<String>,
    pub normal_map: Option<String>,
    pub mask: Option<String>,
    pub tone_map: Option<String>,
    pub detail_map: Option<String>,
    pub environment_map: Option<String>,
    pub multilayer: Option<String>,
    pub specularity_map: Option<String>,
    pub dodt: Option<DecalData>,
    pub flags: Option<TypeFlags>,
}

impl fmt::Display for TextureSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TextureSet ({})",
            self.edid
                .as_ref()
                .map(|value| value.as_str())
                .unwrap_or_default()
        )
    }
}

impl TryFrom<TXST> for TextureSet {
    type Error = Error;

    fn try_from(raw: TXST) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let obnd = OBND::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let color_map = TX00::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let normal_map = TX01::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let mask = TX02::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let tone_map = TX03::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let detail_map = TX04::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let environment_map = TX05::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let multilayer = TX06::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let specularity_map = TX07::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let dodt = DODT::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let flags = DNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        Ok(Self {
            header: raw.header,
            edid,
            obnd,
            color_map,
            normal_map,
            mask,
            tone_map,
            detail_map,
            environment_map,
            multilayer,
            specularity_map,
            dodt,
            flags,
        })
    }
}
