use super::{get_cursor, Flags, RecordHeader};
use crate::error::Error;
use crate::fields::EDID;
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"CSTY")]
pub struct CSTY {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatStyle {
    pub header: RecordHeader,
    pub edid: Option<String>,
}

impl fmt::Display for CombatStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CombatStyle ({})", self.edid.as_deref().unwrap_or("~"))
    }
}

impl TryFrom<CSTY> for CombatStyle {
    type Error = Error;

    fn try_from(raw: CSTY) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        Ok(Self {
            header: raw.header,
            edid,
        })
    }
}
