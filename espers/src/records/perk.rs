use super::{get_cursor, Flags, RecordHeader};
use crate::error::Error;
use crate::fields::{ScriptList, DESC, EDID, FULL, ICON, VMAD};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"PERK")]
pub struct PERK {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Perk {
    pub header: RecordHeader,
    pub edid: String,
    pub scripts: Option<ScriptList>,
    pub full_name: Option<String>,
    pub description: String,
    pub icon: Option<String>,
}

impl fmt::Display for Perk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Perk ({})", self.edid)
    }
}

impl TryFrom<PERK> for Perk {
    type Error = Error;

    fn try_from(raw: PERK) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let scripts: Option<ScriptList> = VMAD::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        let full_name = FULL::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let description = DESC::read(&mut cursor)?.try_into()?;
        let icon = ICON::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        Ok(Self {
            header: raw.header,
            edid,
            scripts,
            full_name,
            description,
            icon,
        })
    }
}
