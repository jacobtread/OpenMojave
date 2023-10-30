use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID, LocalizedString};
use crate::error::Error;
use crate::fields::{BNAM, CNAM, DESC, EDID, FNAM, GNAM, INTV, MNAM};
use binrw::{binrw, BinRead};
use rgb::RGBA8;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"COLL")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct COLL {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,

    #[br(calc(localized))]
    #[bw(ignore)]
    pub localized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollisionLayer {
    pub header: RecordHeader,
    pub edid: String,
    pub description: LocalizedString,
    pub unique_id: u32,
    pub debug_color: RGBA8,
    pub flags: u32,
    pub name: String,
    pub interactables: Vec<FormID>,
}

impl fmt::Display for CollisionLayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CollisionLayer ({})", self.edid)
    }
}

impl TryFrom<COLL> for CollisionLayer {
    type Error = Error;

    fn try_from(raw: COLL) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let description = if raw.localized {
            LocalizedString::Localized(DESC::read(&mut cursor)?.try_into()?)
        } else {
            LocalizedString::ZString(DESC::read(&mut cursor)?.try_into()?)
        };
        let unique_id = BNAM::read(&mut cursor)?.try_into()?;
        let debug_color = FNAM::read(&mut cursor)?.try_into()?;
        let flags = GNAM::read(&mut cursor)?.try_into()?;
        let name = MNAM::read(&mut cursor)?.try_into()?;

        INTV::read(&mut cursor)?;
        let interactables = CNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?
            .unwrap_or_else(Vec::new);

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            description,
            unique_id,
            debug_color,
            flags,
            name,
            interactables,
        })
    }
}
