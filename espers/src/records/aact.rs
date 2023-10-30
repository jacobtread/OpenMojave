use super::{get_cursor, Flags, RecordHeader};
use crate::common::check_done_reading;
use crate::error::Error;
use crate::fields::{CNAM, EDID};
use binrw::{binrw, BinRead, BinWrite};
use rgb::RGBA8;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"AACT")]
pub struct AACT {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub header: RecordHeader,
    pub edid: String,
    pub color: Option<RGBA8>,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Action ({})", self.edid)
    }
}

impl TryFrom<AACT> for Action {
    type Error = Error;

    fn try_from(raw: AACT) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let color = CNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            color,
        })
    }
}

impl TryFrom<Action> for AACT {
    type Error = Error;

    fn try_from(obj: Action) -> Result<Self, Self::Error> {
        let mut data = Cursor::new(Vec::new());
        EDID::try_from(obj.edid)?.write(&mut data)?;

        Ok(Self {
            header: obj.header,
            data: data.into_inner(),
        })
    }
}
