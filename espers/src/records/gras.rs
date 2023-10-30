use super::{get_cursor, Flags, RecordHeader};
use crate::error::Error;
use crate::fields::{ObjectBounds, DATA, EDID, MODL, MODT, OBND};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"GRAS")]
pub struct GRAS {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grass {
    pub header: RecordHeader,
    pub edid: String,
    pub bounds: ObjectBounds,
    pub model_filename: String,
    pub model_textures: Option<MODT>,
    pub data: DATA,
}

impl fmt::Display for Grass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Grass ({})", self.edid)
    }
}

impl TryFrom<GRAS> for Grass {
    type Error = Error;

    fn try_from(raw: GRAS) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let bounds = OBND::read(&mut cursor)?.try_into()?;
        let model_filename = MODL::read(&mut cursor)?.try_into()?;
        let model_textures = MODT::read(&mut cursor).ok();
        let data = DATA::read(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            bounds,
            model_filename,
            model_textures,
            data,
        })
    }
}
