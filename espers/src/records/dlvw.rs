use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use crate::fields::{BNAM, DNAM, EDID, ENAM, QNAM, TNAM};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"DLVW")]
pub struct DLVW {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueView {
    pub header: RecordHeader,
    pub edid: String,
    pub parent_quest: FormID,
    pub branches: Vec<FormID>,
    pub topics: Vec<FormID>,
    pub unknown: Option<u32>,
    pub show_all_text: Option<u8>,
}

impl fmt::Display for DialogueView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DialogueView ({})", self.edid)
    }
}

impl TryFrom<DLVW> for DialogueView {
    type Error = Error;

    fn try_from(raw: DLVW) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let parent_quest = QNAM::read(&mut cursor)?.try_into()?;
        let mut branches = Vec::new();
        while let Ok(b) = BNAM::read(&mut cursor) {
            branches.push(b.try_into()?);
        }
        let mut topics = Vec::new();
        while let Ok(t) = TNAM::read(&mut cursor) {
            topics.push(t.try_into()?);
        }
        let unknown = ENAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let show_all_text = DNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            parent_quest,
            branches,
            topics,
            unknown,
            show_all_text,
        })
    }
}
