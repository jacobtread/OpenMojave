use super::{get_cursor, Flags, RecordHeader};
use crate::common::FormID;
use crate::error::Error;
use crate::fields::{ScriptList, EDID, NAME, VMAD};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"REFR")]
pub struct REFR {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectRef {
    pub header: RecordHeader,
    pub edid: Option<String>,
    pub scripts: Option<ScriptList>,
    pub name: FormID,
}

impl fmt::Display for ObjectRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ObjectRef ({})",
            self.edid.as_ref().unwrap_or(&format!("{}", self.name)),
        )
    }
}

impl TryFrom<REFR> for ObjectRef {
    type Error = Error;

    fn try_from(raw: REFR) -> Result<Self, Self::Error> {
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
        let name = NAME::read(&mut cursor)?.try_into()?;

        Ok(Self {
            header: raw.header,
            edid,
            scripts,
            name,
        })
    }
}
