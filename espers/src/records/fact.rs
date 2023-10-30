use super::{get_cursor, Flags, RecordHeader};
use crate::common::FormID;
use crate::error::Error;
use crate::fields::{
    CrimeGold, CRGR, CRVA, DATA, EDID, FULL, JAIL, JOUT, PLCN, STOL, VENV, WAIT, XNAM,
};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"FACT")]
pub struct FACT {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Faction {
    pub header: RecordHeader,
    pub edid: String,
    pub full_name: Option<String>,
    pub xnams: Vec<XNAM>,
    pub flags: DATA,
    pub jail: Option<FormID>,
    pub follower_wait_marker: Option<FormID>,
    pub evidence_chest: Option<FormID>,
    pub belongings_chest: Option<FormID>,
    pub crime_group: Option<FormID>,
    pub jail_outfit: Option<FormID>,
    pub crime_gold: Option<CrimeGold>,
    pub vendor: Option<VENV>,
}

impl fmt::Display for Faction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Faction ({})", self.edid)
    }
}

impl TryFrom<FACT> for Faction {
    type Error = Error;

    fn try_from(raw: FACT) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let full_name = FULL::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let mut xnams = Vec::new();

        while let Ok(x) = XNAM::read(&mut cursor) {
            xnams.push(x);
        }

        let flags = DATA::read(&mut cursor)?;
        let jail = JAIL::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let follower_wait_marker = WAIT::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let evidence_chest = STOL::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let belongings_chest = PLCN::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let crime_group = CRGR::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let jail_outfit = JOUT::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let crime_gold = CRVA::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let vendor = VENV::read(&mut cursor).ok();

        Ok(Self {
            header: raw.header,
            edid,
            full_name,
            xnams,
            flags,
            jail,
            follower_wait_marker,
            evidence_chest,
            belongings_chest,
            crime_group,
            jail_outfit,
            crime_gold,
            vendor,
        })
    }
}
