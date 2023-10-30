use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use crate::fields::{DNAM, EDID, QNAM, SNAM, TNAM};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"DLBR")]
pub struct DLBR {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueBranch {
    pub header: RecordHeader,
    pub edid: String,
    pub quest_id: FormID,
    pub unknown: u32,
    pub flags: u32,
    pub start_dialog: FormID,
}

impl fmt::Display for DialogueBranch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DialogBranch ({})", self.edid)
    }
}

impl TryFrom<DLBR> for DialogueBranch {
    type Error = Error;

    fn try_from(raw: DLBR) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let quest_id = QNAM::read(&mut cursor)?.try_into()?;
        let unknown = TNAM::read(&mut cursor)?.try_into()?;
        let flags = DNAM::read(&mut cursor)?.try_into()?;
        let start_dialog = SNAM::read(&mut cursor)?.try_into()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            quest_id,
            unknown,
            flags,
            start_dialog,
        })
    }
}
