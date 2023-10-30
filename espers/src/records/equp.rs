use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use crate::fields::{DATA, EDID, PNAM};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"EQUP")]
pub struct EQUP {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipSlot {
    pub header: RecordHeader,
    pub edid: String,
    pub equip_slots: Vec<FormID>,
    pub use_all_parents: u32,
}

impl fmt::Display for EquipSlot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EquipSlot ({})", self.edid)
    }
}

impl TryFrom<EQUP> for EquipSlot {
    type Error = Error;

    fn try_from(raw: EQUP) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let equip_slots = PNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?
            .unwrap_or_else(Vec::new);
        let use_all_parents = DATA::read(&mut cursor)?.try_into()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            equip_slots,
            use_all_parents,
        })
    }
}
