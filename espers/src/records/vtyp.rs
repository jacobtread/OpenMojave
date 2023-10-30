use super::{get_cursor, Flags, RecordHeader};
use crate::error::Error;
use crate::fields::{DNAM, EDID};
use binrw::{binrw, BinRead};
use bitflags::bitflags;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

bitflags! {
    #[binrw]
    #[brw(little)]
    #[derive(Deserialize, Serialize)]
    pub struct TypeFlags: u8 {
        const ALLOW_DEFAULT_DIALOG = 0x01;
        const FEMALE = 0x02;
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
#[brw(little, magic = b"VTYP")]
pub struct VTYP {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceType {
    pub header: RecordHeader,
    pub edid: String,
    pub flags: TypeFlags,
}

impl fmt::Display for VoiceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VoiceType ({})", self.edid)
    }
}

impl TryFrom<VTYP> for VoiceType {
    type Error = Error;

    fn try_from(raw: VTYP) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let kind = DNAM::read(&mut cursor)?.try_into()?;

        Ok(Self {
            header: raw.header,
            edid,
            flags: kind,
        })
    }
}
