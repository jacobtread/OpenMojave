use super::{get_cursor, Flags, RecordHeader};
use crate::error::Error;
use crate::fields::EDID;
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"IPDS")]
pub struct IPDS {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactDataSet {
    pub header: RecordHeader,
    pub edid: Option<String>,
}

impl fmt::Display for ImpactDataSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ImpactDataSet ({})", self.edid.as_deref().unwrap_or("~"))
    }
}

impl TryFrom<IPDS> for ImpactDataSet {
    type Error = Error;

    fn try_from(raw: IPDS) -> Result<Self, Self::Error> {
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
