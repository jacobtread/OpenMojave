use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID, LocalizedString};
use crate::error::Error;
use crate::fields::{
    Model, ObjectBounds, ScriptList, ANAM, BNAM, EDID, FNAM, FULL, OBND, SNAM, TNAM, VMAD,
};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"DOOR")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DOOR {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,

    #[br(calc(localized))]
    #[bw(ignore)]
    pub localized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Door {
    pub header: RecordHeader,
    pub edid: Option<String>,
    pub scripts: Option<ScriptList>,
    pub bounds: ObjectBounds,
    pub full_name: Option<LocalizedString>,
    pub model: Option<Model>,
    pub open_sound: Option<FormID>,
    pub close_sound: Option<FormID>,
    pub loop_sound: Option<FormID>,
    pub flags: u8,
    pub random_teleports: Vec<FormID>,
}

impl fmt::Display for Door {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Door ({})", self.edid.as_deref().unwrap_or("~"))
    }
}

impl TryFrom<DOOR> for Door {
    type Error = Error;

    fn try_from(raw: DOOR) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let scripts = VMAD::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let bounds = OBND::read(&mut cursor)?.try_into()?;
        let full_name = match (FULL::read(&mut cursor), raw.localized) {
            (Ok(f), true) => Some(LocalizedString::Localized(f.try_into()?)),
            (Ok(z), false) => Some(LocalizedString::ZString(z.try_into()?)),
            (Err(_), _) => None,
        };
        let model = Model::try_load(&mut cursor, raw.header.internal_version)?;
        let open_sound = SNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let close_sound = ANAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let loop_sound = BNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let flags = FNAM::read(&mut cursor)?.try_into()?;
        let mut random_teleports = Vec::new();
        while let Ok(rt) = TNAM::read(&mut cursor) {
            random_teleports.push(rt.try_into()?);
        }

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            scripts,
            bounds,
            full_name,
            model,
            open_sound,
            close_sound,
            loop_sound,
            flags,
            random_teleports,
        })
    }
}
