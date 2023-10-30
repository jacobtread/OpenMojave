use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use crate::fields::{ObjectBounds, Unknown4, DATA, DNAM, EDID, MODL, MODT, OBND, SNAM};
use binrw::{binrw, BinRead};
use bitflags::bitflags;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

bitflags! {
    #[binrw]
    #[brw(little)]
    #[derive(Deserialize, Serialize)]
    pub struct AddonNodeFlags: u16 {
        const UNKNOWN = 0x0001;
        const ALWAYS_LOADED = 0x0002;
    }
}

impl TryFrom<DNAM> for (u16, AddonNodeFlags) {
    type Error = Error;

    fn try_from(raw: DNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"ADDN")]
pub struct ADDN {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddonNode {
    pub header: RecordHeader,
    pub edid: String,
    pub bounds: ObjectBounds,
    pub model_filename: String,
    pub model_textures: Vec<Unknown4>,
    pub addon_node_index: u32,
    pub ambient_sound: Option<FormID>,
    pub particle_system_cap: u16,
    pub flags: AddonNodeFlags,
}

impl fmt::Display for AddonNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AddonNode ({})", self.edid)
    }
}

impl TryFrom<ADDN> for AddonNode {
    type Error = Error;

    fn try_from(raw: ADDN) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let bounds = OBND::read(&mut cursor)?.try_into()?;
        let model_filename = MODL::read(&mut cursor)?.try_into()?;
        let modt = MODT::read(&mut cursor)?;
        let model_textures = match modt.clone().try_into() {
            Ok(modt) => modt,
            Err(err) => panic!("{} - {:?}", err, modt.data),
        };
        let addon_node_index = DATA::read(&mut cursor)?.try_into()?;
        let ambient_sound = SNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let (particle_system_cap, flags) = DNAM::read(&mut cursor)?.try_into()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            bounds,
            model_filename,
            model_textures,
            addon_node_index,
            ambient_sound,
            particle_system_cap,
            flags,
        })
    }
}
