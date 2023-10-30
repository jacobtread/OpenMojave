use crate::common::{check_done_reading, FormID, WString};
use crate::error::Error;
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::io::{Cursor, Read};

#[binrw]
#[brw(little, magic = b"VMAD")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VMAD {
    pub size: u16,
    #[br(count = size)]
    pub data: Vec<u8>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RawPropertyType {
    #[brw(magic = 1u8)]
    Object { status: u8, data: [u32; 2] },
    #[brw(magic = 2u8)]
    String { status: u8, data: WString },
    #[brw(magic = 3u8)]
    Int { status: u8, data: i32 },
    #[brw(magic = 4u8)]
    Float { status: u8, data: f32 },
    #[brw(magic = 5u8)]
    Bool { status: u8, data: u8 },
    #[brw(magic = 11u8)]
    ObjectList {
        status: u8,
        count: u32,
        #[br(count = count)]
        data: Vec<[u32; 2]>,
    },
    #[brw(magic = 12u8)]
    StringList {
        status: u8,
        count: u32,
        #[br(count = count)]
        data: Vec<WString>,
    },
    #[brw(magic = 13u8)]
    IntList {
        status: u8,
        count: u32,
        #[br(count = count)]
        data: Vec<i32>,
    },
    #[brw(magic = 14u8)]
    FloatList {
        status: u8,
        count: u32,
        #[br(count = count)]
        data: Vec<f32>,
    },
    #[brw(magic = 14u8)]
    BoolList {
        status: u8,
        count: u32,
        #[br(count = count)]
        data: Vec<u8>,
    },
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawProperty {
    pub name_size: u16,
    #[br(count = name_size)]
    pub name: Vec<u8>,

    pub kind: RawPropertyType,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawScript {
    pub name_size: u16,
    #[br(count = name_size)]
    pub name: Vec<u8>,

    pub status: u8,
    pub property_count: u16,

    #[br(count = property_count)]
    pub properties: Vec<RawProperty>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawScriptList {
    pub version: u16,
    pub object_format: u16,
    pub script_count: u16,

    #[br(count = script_count)]
    pub scripts: Vec<RawScript>,
}

impl TryFrom<VMAD> for RawScriptList {
    type Error = Error;

    fn try_from(raw: VMAD) -> Result<RawScriptList, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        Ok(RawScriptList::read(&mut cursor)?)
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerkFragment {
    pub index: u16,
    pub unknown1: i16,
    pub unknown2: i8,
    pub script_name: WString,
    pub fragment_name: WString,
}

#[binrw]
#[brw(little, magic = 2u8)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerkFragmentList {
    pub filename: WString,
    pub fragment_count: u16,
    #[br(count = fragment_count)]
    pub fragments: Vec<PerkFragment>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum FragmentList {
    Perk(PerkFragmentList),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Property {
    ObjectV1 {
        name: String,
        status: u8,
        form_id: FormID,
        alias: i16,
        unused: u16,
    },
    ObjectV2 {
        name: String,
        status: u8,
        unused: u16,
        alias: i16,
        form_id: FormID,
    },
    String {
        name: String,
        status: u8,
        value: String,
    },
    Int {
        name: String,
        status: u8,
        value: i32,
    },
    Float {
        name: String,
        status: u8,
        value: f32,
    },
    Bool {
        name: String,
        status: u8,
        value: bool,
    },
    PackedObjectV1 {
        form_id: FormID,
        alias: i16,
        unused: u16,
    },
    PackedObjectV2 {
        unused: u16,
        alias: i16,
        form_id: FormID,
    },
    PackedString {
        value: String,
    },
    PackedInt {
        value: i32,
    },
    PackedFloat {
        value: f32,
    },
    PackedBool {
        value: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    pub name: String,
    pub status: u8,
    pub properties: Vec<Property>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptList {
    pub version: u16,
    pub object_format: u16,
    pub scripts: Vec<Script>,
    pub fragments: Option<FragmentList>,
}

fn helper(p: RawProperty, object_format: u16) -> Vec<Property> {
    match (p.kind, object_format) {
        (RawPropertyType::Object { status, data }, 1) => vec![Property::ObjectV1 {
            name: String::from_utf8_lossy(&p.name).into(),
            status,
            form_id: FormID(data[0]),
            alias: (data[1] & 0xFFFF) as i16,
            unused: (data[1] >> 0x10) as u16,
        }],
        (RawPropertyType::Object { status, data }, 2) => vec![Property::ObjectV2 {
            name: String::from_utf8_lossy(&p.name).into(),
            status,
            unused: (data[0] & 0xFFFF) as u16,
            alias: (data[0] >> 0x10) as i16,
            form_id: FormID(data[1]),
        }],
        (RawPropertyType::Object { .. }, _) => {
            unreachable!("Invalid Object Format Version")
        }
        (RawPropertyType::String { status, data }, _) => vec![Property::String {
            name: String::from_utf8_lossy(&p.name).into(),
            status,
            value: data.to_string(),
        }],
        (RawPropertyType::Int { status, data }, _) => vec![Property::Int {
            name: String::from_utf8_lossy(&p.name).into(),
            status,
            value: data,
        }],
        (RawPropertyType::Float { status, data }, _) => vec![Property::Float {
            name: String::from_utf8_lossy(&p.name).into(),
            status,
            value: data,
        }],
        (RawPropertyType::Bool { status, data }, _) => vec![Property::Bool {
            name: String::from_utf8_lossy(&p.name).into(),
            status,
            value: data == 1,
        }],
        (RawPropertyType::ObjectList { data, .. }, 1) => data
            .into_iter()
            .map(|data| Property::PackedObjectV1 {
                form_id: FormID(data[0]),
                alias: (data[1] & 0xFFFF) as i16,
                unused: (data[1] >> 0x10) as u16,
            })
            .collect(),
        (RawPropertyType::ObjectList { data, .. }, 2) => data
            .into_iter()
            .map(|data| Property::PackedObjectV2 {
                unused: (data[0] & 0xFFFF) as u16,
                alias: (data[0] >> 0x10) as i16,
                form_id: FormID(data[1]),
            })
            .collect(),
        (RawPropertyType::ObjectList { .. }, _) => {
            unreachable!("Invalid Object Format Version")
        }
        (RawPropertyType::StringList { data, .. }, _) => data
            .into_iter()
            .map(|value| Property::PackedString {
                value: value.to_string(),
            })
            .collect(),
        (RawPropertyType::IntList { data, .. }, _) => data
            .into_iter()
            .map(|value| Property::PackedInt { value })
            .collect(),
        (RawPropertyType::FloatList { data, .. }, _) => data
            .into_iter()
            .map(|value| Property::PackedFloat { value })
            .collect(),
        (RawPropertyType::BoolList { data, .. }, _) => data
            .into_iter()
            .map(|value| Property::PackedBool { value: value == 1 })
            .collect(),
    }
}

impl TryInto<ScriptList> for VMAD {
    type Error = Error;

    fn try_into(self) -> Result<ScriptList, Self::Error> {
        let mut cursor = Cursor::new(&self.data);
        let raw_scripts = RawScriptList::read(&mut cursor)?;
        let scripts: Result<_, _> = raw_scripts
            .scripts
            .into_iter()
            .map(|s| -> Result<Script, Error> {
                Ok(Script {
                    name: String::from_utf8_lossy(&s.name).into(),
                    status: s.status,
                    properties: s
                        .properties
                        .into_iter()
                        .flat_map(|p| helper(p, raw_scripts.object_format))
                        .collect(),
                })
            })
            .collect();

        let mut fragments = Vec::new();
        cursor.read_to_end(&mut fragments)?;
        let fragments = if !fragments.is_empty() {
            let mut cursor = Cursor::new(&fragments);
            let fragments = FragmentList::read(&mut cursor)?;
            check_done_reading(&mut cursor)?;
            Some(fragments)
        } else {
            None
        };

        check_done_reading(&mut cursor)?;

        Ok(ScriptList {
            version: raw_scripts.version,
            object_format: raw_scripts.object_format,
            scripts: scripts?,
            fragments,
        })
    }
}
