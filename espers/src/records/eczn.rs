use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use crate::fields::{DATA, EDID};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"ECZN")]
pub struct ECZN {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterZoneData {
    pub owner_id: FormID,
    pub location: FormID,
    pub owner_rank: i8,
    pub min_level: u8,
    pub flags: u8,
    pub max_level: u8,
}

impl TryFrom<DATA> for EncounterZoneData {
    type Error = Error;

    fn try_from(raw: DATA) -> Result<Self, Self::Error> {
        if raw.data == [0; 8] {
            return Ok(Self {
                owner_id: FormID(0),
                location: FormID(0),
                owner_rank: 0,
                min_level: 0,
                flags: 0,
                max_level: 0,
            });
        }
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterZone {
    pub header: RecordHeader,
    pub edid: String,
    pub data: EncounterZoneData,
}

impl fmt::Display for EncounterZone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EncounterZone ({})", self.edid)
    }
}

impl TryFrom<ECZN> for EncounterZone {
    type Error = Error;

    fn try_from(raw: ECZN) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let data = DATA::read(&mut cursor)?.try_into()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            data,
        })
    }
}
