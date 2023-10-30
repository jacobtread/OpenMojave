use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID, LocalizedString};
use crate::error::Error;
use crate::fields::{Model, ObjectBounds, DATA, EDID, EITM, FULL, MNAM, OBND};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"EXPL")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EXPL {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,

    #[br(calc(localized))]
    #[bw(ignore)]
    pub localized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Explosion {
    pub header: RecordHeader,
    pub edid: String,
    pub bounds: ObjectBounds,
    pub full_name: Option<LocalizedString>,
    pub model: Option<Model>,
    pub enchantment: Option<FormID>,
    pub modifier: Option<FormID>,
    pub data: Vec<u8>,
}

impl fmt::Display for Explosion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Explosion ({})", self.edid)
    }
}

impl TryFrom<EXPL> for Explosion {
    type Error = Error;

    fn try_from(raw: EXPL) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let bounds = OBND::read(&mut cursor)?.try_into()?;
        let full_name = match (FULL::read(&mut cursor), raw.localized) {
            (Ok(f), true) => Some(LocalizedString::Localized(f.try_into()?)),
            (Ok(z), false) => Some(LocalizedString::ZString(z.try_into()?)),
            (Err(_), _) => None,
        };
        let model = Model::try_load(&mut cursor, raw.header.internal_version)?;
        let enchantment = EITM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let modifier = MNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let data = DATA::read(&mut cursor)?.try_into()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            bounds,
            full_name,
            model,
            enchantment,
            modifier,
            data,
        })
    }
}
