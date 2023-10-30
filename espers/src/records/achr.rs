use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use crate::fields::{
    ScriptList, DATA, EDID, INAM, NAME, PDTO, VMAD, XAPD, XAPR, XESP, XEZN, XHOR, XIS2, XLCM, XLCN,
    XLKR, XLRL, XLRT, XOWN, XPPA, XPRD, XRGB, XRGD, XSCL,
};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum XRec {
    INAM(INAM),
    PDTO(PDTO),
    XAPD(XAPD),
    XAPR(XAPR),
    XESP(XESP),
    XEZN(XEZN),
    XHOR(XHOR),
    XIS2(XIS2),
    XLCM(XLCM),
    XLCN(XLCN),
    XLKR(XLKR),
    XLRL(XLRL),
    XLRT(XLRT),
    XOWN(XOWN),
    XPPA(XPPA),
    XPRD(XPRD),
    XRGB(XRGB),
    XRGD(XRGD),
    XSCL(XSCL),
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Coords {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rx: f32,
    pub ry: f32,
    pub rz: f32,
}

impl TryFrom<DATA> for Coords {
    type Error = Error;

    fn try_from(raw: DATA) -> Result<Self, Self::Error> {
        Ok(Self::read(&mut Cursor::new(&raw.data))?)
    }
}

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"ACHR")]
pub struct ACHR {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorRef {
    pub header: RecordHeader,
    pub edid: Option<String>,
    pub scripts: Option<ScriptList>,
    pub name: FormID,
    pub encounter_zone: Option<FormID>,
    pub patrol_idle: Option<f32>,
    pub unknown1: Option<()>,
    pub unknown2: Option<FormID>,
    pub topic_data: Option<PDTO>,
    pub ragdoll: Option<XRGD>,
    pub unknown3: Option<XRGB>,
    pub leveled_creature_data: Option<u32>,
    pub activation_parent_flags: Option<u8>,
    pub activate_parent: Vec<XAPR>,
    pub location_ref_type: Option<FormID>,
    pub horse_id: Option<FormID>,
    pub enable_parent: Option<XESP>,
    pub owner: Option<FormID>,
    pub location: Option<FormID>,
    pub location_route: Vec<XLKR>,
    pub xis2: Option<XIS2>,
    pub xlrl: Option<FormID>,
    pub scale: Option<f32>,
    pub coords: Option<Coords>,
}

impl fmt::Display for ActorRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ActorRef ({})", self.edid.as_deref().unwrap_or("~"))
    }
}

impl TryFrom<ACHR> for ActorRef {
    type Error = Error;

    fn try_from(raw: ACHR) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        let scripts = VMAD::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        let name = NAME::read(&mut cursor)?.try_into()?;

        let mut encounter_zone = None;
        let mut patrol_idle = None;
        let mut unknown1 = None;
        let mut unknown2 = None;
        let mut topic_data = None;
        let mut ragdoll = None;
        let mut unknown3 = None;
        let mut leveled_creature_data = None;
        let mut activation_parent_flags = None;
        let mut activate_parent = Vec::new();
        let mut location_ref_type = None;
        let mut horse_id = None;
        let mut enable_parent = None;
        let mut owner = None;
        let mut location = None;
        let mut location_route = Vec::new();
        let mut xis2 = None;
        let mut xlrl = None;
        let mut scale = None;

        // There appears to be no set order for these records, so just
        // loop over them and parse them as they come.
        while let Ok(rec) = XRec::read(&mut cursor) {
            match rec {
                XRec::INAM(r) => match unknown2 {
                    Some(x) => return Err(Self::Error::DuplicateField(format!("{}", x))),
                    None => unknown2 = Some(r.try_into()?),
                },
                XRec::PDTO(r) => match topic_data {
                    Some(x) => return Err(Self::Error::DuplicateField(format!("{:?}", x))),
                    None => topic_data = Some(r),
                },
                XRec::XAPD(r) => match activation_parent_flags {
                    Some(x) => return Err(Self::Error::DuplicateField(format!("{}", x))),
                    None => activation_parent_flags = Some(r.try_into()?),
                },
                XRec::XAPR(r) => activate_parent.push(r),
                XRec::XESP(r) => match enable_parent {
                    Some(x) => return Err(Self::Error::DuplicateField(format!("{:?}", x))),
                    None => enable_parent = Some(r),
                },
                XRec::XEZN(r) => match encounter_zone {
                    Some(x) => return Err(Self::Error::DuplicateField(format!("{}", x))),
                    None => encounter_zone = Some(r.try_into()?),
                },
                XRec::XHOR(r) => match horse_id {
                    Some(x) => return Err(Self::Error::DuplicateField(format!("{}", x))),
                    None => horse_id = Some(r.try_into()?),
                },
                XRec::XIS2(r) => match xis2 {
                    Some(x) => return Err(Self::Error::DuplicateField(format!("{:?}", x))),
                    None => xis2 = Some(r),
                },
                XRec::XLCM(r) => match leveled_creature_data {
                    Some(x) => return Err(Self::Error::DuplicateField(format!("{}", x))),
                    None => leveled_creature_data = Some(r.try_into()?),
                },
                XRec::XLCN(r) => match location {
                    Some(x) => return Err(Self::Error::DuplicateField(format!("{}", x))),
                    None => location = Some(r.try_into()?),
                },
                XRec::XLKR(r) => location_route.push(r),
                XRec::XLRL(r) => match xlrl {
                    Some(x) => return Err(Self::Error::DuplicateField(format!("{}", x))),
                    None => xlrl = Some(r.try_into()?),
                },
                XRec::XLRT(r) => match location_ref_type {
                    Some(x) => return Err(Self::Error::DuplicateField(format!("{}", x))),
                    None => location_ref_type = Some(r.try_into()?),
                },
                XRec::XOWN(r) => match owner {
                    Some(x) => return Err(Self::Error::DuplicateField(format!("{}", x))),
                    None => owner = Some(r.try_into()?),
                },
                XRec::XPPA(r) => match unknown1 {
                    Some(x) => return Err(Self::Error::DuplicateField(format!("{:?}", x))),
                    None => unknown1 = Some(r.try_into()?),
                },
                XRec::XPRD(r) => match patrol_idle {
                    Some(x) => return Err(Self::Error::DuplicateField(format!("{}", x))),
                    None => patrol_idle = Some(r.try_into()?),
                },
                XRec::XRGB(r) => match unknown3 {
                    Some(x) => return Err(Self::Error::DuplicateField(format!("{:?}", x))),
                    None => unknown3 = Some(r),
                },
                XRec::XRGD(r) => match ragdoll {
                    Some(x) => return Err(Self::Error::DuplicateField(format!("{:?}", x))),
                    None => ragdoll = Some(r),
                },
                XRec::XSCL(r) => match scale {
                    Some(x) => return Err(Self::Error::DuplicateField(format!("{}", x))),
                    None => scale = Some(r.try_into()?),
                },
            }
        }

        let coords = DATA::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            scripts,
            name,
            coords,
            encounter_zone,
            patrol_idle,
            unknown1,
            unknown2,
            topic_data,
            ragdoll,
            unknown3,
            leveled_creature_data,
            activation_parent_flags,
            activate_parent,
            location_ref_type,
            horse_id,
            enable_parent,
            owner,
            location,
            location_route,
            xis2,
            xlrl,
            scale,
        })
    }
}
