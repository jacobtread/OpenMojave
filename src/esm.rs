use binrw::binrw;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct Flags: u32 {
        const MASTER = 0x00000001;
        const DELETED_GROUP = 0x00000010;
        const DELETED_RECORD = 0x00000010;
        const LOCALIZED = 0x00000080;
        const LIGHT_MASTER = 0x00000200;
        const COMPRESSED = 0x00040000;
    }
}

// types: "ENCH", "TERM", "GLOB", "HAIR", "NOTE", "PROJ", "ALCH", "AMMO", "FACT", "LVLN", "RACE", "CELL", "TES4", "DOOR", "GRUP", "BOOK", "STAT", "FURN", "CREA", "EDID", "SCPT", "ACTI", "MSTT", "TXST", "GRAS", "ASPC", "REGN", "LIGH", "WTHR"

#[binrw]
#[brw(little)]
#[derive(Clone, Deserialize, Serialize)]
pub struct RecordHeader {
    pub ty: u32,
    pub size: u32,
    pub flags: u32,
    pub form_id: u32,
    pub timestamp: u16,
    pub version_control: u16,
    pub internal_version: u16,
    pub unknown: u16,
}

impl core::fmt::Debug for RecordHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            RecordHeader {
                ty: ty,
                size: size,
                flags: flags,
                form_id: form_id,
                timestamp: timestamp,
                version_control: version_control,
                internal_version: internal_version,
                unknown: unknown,
            } => f
                .debug_struct("RecordHeader")
                .field("ty", &unsafe {
                    std::str::from_utf8_unchecked(&ty.to_be_bytes())
                })
                .field("size", &size)
                .field("flags", &flags)
                .field("form_id", &form_id)
                .field("timestamp", &timestamp)
                .field("version_control", &version_control)
                .field("internal_version", &internal_version)
                .field("unknown", &unknown)
                .finish(),
        }
    }
}
