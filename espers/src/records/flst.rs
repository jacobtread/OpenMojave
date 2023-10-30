use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use crate::fields::{EDID, LNAM};
use binrw::{binrw, until_eof, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"FLST")]
pub struct FLST {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormList {
    pub header: RecordHeader,
    pub edid: String,
    pub objects: Vec<FormID>,
}

impl fmt::Display for FormList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FormList ({})", self.edid)
    }
}

impl TryFrom<FLST> for FormList {
    type Error = Error;

    fn try_from(raw: FLST) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let objects: Vec<LNAM> = until_eof(&mut cursor, binrw::Endian::Little, ())?;
        let objects: Result<_, _> = objects.into_iter().map(TryInto::try_into).collect();

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            objects: objects?,
        })
    }
}
