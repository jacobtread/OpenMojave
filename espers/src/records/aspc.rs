use super::{get_cursor, Flags, RecordHeader};
use crate::common::FormID;
use crate::error::Error;
use crate::fields::{ObjectBounds, BNAM, EDID, OBND, RDAT, SNAM};
use binrw::binrw;
use binrw::BinRead;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"ASPC")]
pub struct ASPC {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcousticSpace {
    pub header: RecordHeader,
    pub edid: String,
    pub bounds: ObjectBounds,
    pub ambient: Option<FormID>,
    pub region_data: Option<FormID>,
    pub reverb: Option<FormID>,
}

impl fmt::Display for AcousticSpace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AcousticSpace ({})", self.edid)
    }
}

impl TryFrom<ASPC> for AcousticSpace {
    type Error = Error;

    fn try_from(raw: ASPC) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let bounds = OBND::read(&mut cursor)?.try_into()?;
        let ambient = SNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let region_data = RDAT::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let reverb = BNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        Ok(Self {
            header: raw.header,
            edid,
            bounds,
            ambient,
            region_data,
            reverb,
        })
    }
}
