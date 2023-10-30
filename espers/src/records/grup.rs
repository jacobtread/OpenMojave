use crate::error::Error;
use crate::records::{RawRecord, Record};
use crate::string_table::StringTables;
use binrw::{binrw, io::Cursor, until_eof, Endian};
use bitflags::bitflags;
use serde_derive::{Deserialize, Serialize};
use std::fmt;

bitflags! {
    #[binrw]
    #[derive(Deserialize, Serialize)]
    struct Flags: u32 {
        const MASTER = 0x00000001;
        const LOCALIZED = 0x00000080;
        const LIGHT_MASTER = 0x00000200;
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupHeader {
    pub size: u32,
    pub label: u32,
    pub kind: i32,
    pub timestamp: u16,
    pub version_control_info: u16,
    pub unknown: u32,
}

#[binrw]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[brw(little, magic = b"GRUP")]
#[br(import(localized: bool))]
pub struct GRUP {
    pub header: GroupHeader,

    #[br(count = header.size - 24)]
    pub data: Vec<u8>,

    #[br(calc(localized))]
    #[bw(ignore)]
    pub localized: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Group {
    pub header: GroupHeader,
    pub records: Vec<Record>,
}

impl Group {
    pub fn localize(&mut self, string_table: &StringTables) {
        for record in &mut self.records {
            record.localize(string_table);
        }
    }

    pub fn magics(&self) -> Vec<String> {
        let mut magics = Vec::new();

        for rec in &self.records {
            let magic = String::from_utf8_lossy(&rec.magic()).to_string();
            if !magics.contains(&magic) {
                magics.push(magic);
            }
        }

        magics
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Group ({} items)", self.records.len())
    }
}

impl TryFrom<GRUP> for Group {
    type Error = Error;

    fn try_from(raw: GRUP) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let args = (raw.localized,);
        let recs: Vec<RawRecord> = until_eof(&mut cursor, Endian::Little, args)?;
        let records: Result<Vec<Record>, _> = recs.into_iter().map(Record::try_from).collect();

        Ok(Self {
            header: raw.header,
            records: records?,
        })
    }
}

impl TryFrom<Group> for GRUP {
    type Error = Error;

    fn try_from(obj: Group) -> Result<Self, Self::Error> {
        let data = Cursor::new(Vec::new());

        Ok(Self {
            header: obj.header,
            data: data.into_inner(),
            localized: false,
        })
    }
}
