use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID, LocalizedString};
use crate::error::Error;
use crate::fields::{
    DestructionData, ObjectBounds, ScriptList, Textures, DATA, DESC, EDID, FULL, ICON, MICO, MODL,
    MODT, OBND, QUAL, VMAD, YNAM, ZNAM,
};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"APPA")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APPA {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,

    #[br(calc(localized))]
    #[bw(ignore)]
    pub localized: bool,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApparatusData {
    pub value: u32,
    pub weight: f32,
}

impl TryFrom<DATA> for ApparatusData {
    type Error = Error;

    fn try_from(raw: DATA) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Apparatus {
    pub header: RecordHeader,
    pub edid: String,
    pub scripts: Option<ScriptList>,
    pub bounds: ObjectBounds,
    pub full_name: LocalizedString,
    pub model_filename: Option<String>,
    pub model_textures: Option<Textures>,
    pub icon: Option<String>,
    pub message_icon: Option<String>,
    pub destruction_data: Option<DestructionData>,
    pub pickup_sound: Option<FormID>,
    pub drop_sound: Option<FormID>,
    pub quality: u32,
    pub description: LocalizedString,
    pub data: ApparatusData,
}

impl fmt::Display for Apparatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Apparatus ({})", self.edid)
    }
}

impl TryFrom<APPA> for Apparatus {
    type Error = Error;

    fn try_from(raw: APPA) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let scripts = VMAD::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let bounds = OBND::read(&mut cursor)?.try_into()?;

        let full_name = if raw.localized {
            LocalizedString::Localized(FULL::read(&mut cursor)?.try_into()?)
        } else {
            LocalizedString::ZString(FULL::read(&mut cursor)?.try_into()?)
        };
        let model_filename = MODL::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let model_textures = MODT::read(&mut cursor)
            .ok()
            .map(|modt| Textures::load(modt, raw.header.internal_version))
            .transpose()?;
        let icon = ICON::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let message_icon = MICO::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let destruction_data = DestructionData::load(&mut cursor)?;
        let pickup_sound = YNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let drop_sound = ZNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let quality = QUAL::read(&mut cursor)?.try_into()?;
        let description = if raw.localized {
            LocalizedString::Localized(DESC::read(&mut cursor)?.try_into()?)
        } else {
            LocalizedString::ZString(DESC::read(&mut cursor)?.try_into()?)
        };
        let data = DATA::read(&mut cursor)?.try_into()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            scripts,
            bounds,
            full_name,
            model_filename,
            model_textures,
            icon,
            message_icon,
            destruction_data,
            pickup_sound,
            drop_sound,
            quality,
            description,
            data,
        })
    }
}
