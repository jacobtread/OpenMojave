use super::{get_cursor, Flags, RecordHeader};
use crate::common::{FormID, LocalizedString};
use crate::error::Error;
use crate::fields::{
    DestructionData, ObjectBounds, Textures, DATA, DESC, EDID, FULL, ICON, KSIZ, KWDA, MICO, MODL,
    MODT, OBND, ONAM, YNAM, ZNAM,
};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"AMMO")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AMMO {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,

    #[br(calc(localized))]
    #[bw(ignore)]
    pub localized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ammo {
    pub header: RecordHeader,
    pub edid: Option<String>,
    pub bounds: ObjectBounds,
    pub full_name: Option<LocalizedString>,
    pub model_filename: Option<String>,
    pub model_textures: Option<Textures>,
    pub icon: Option<String>,
    pub message_icon: Option<String>,
    pub destruction_data: Option<DestructionData>,
    pub pickup_sound: Option<FormID>,
    pub drop_sound: Option<FormID>,
    pub description: LocalizedString,
    pub keywords: Vec<FormID>,
    pub data: DATA,
    pub short_name: Option<LocalizedString>,
}

impl fmt::Display for Ammo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ammo ({})", self.edid.as_deref().unwrap_or("~"))
    }
}

impl TryFrom<AMMO> for Ammo {
    type Error = Error;

    fn try_from(raw: AMMO) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let bounds = OBND::read(&mut cursor)?.try_into()?;
        let full_name = match (FULL::read(&mut cursor), raw.localized) {
            (Ok(f), true) => Some(LocalizedString::Localized(f.try_into()?)),
            (Ok(z), false) => Some(LocalizedString::ZString(z.try_into()?)),
            (Err(_), _) => None,
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
        let description = if raw.localized {
            LocalizedString::Localized(DESC::read(&mut cursor)?.try_into()?)
        } else {
            LocalizedString::ZString(DESC::read(&mut cursor)?.try_into()?)
        };

        let keyword_count: Option<u32> = KSIZ::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let mut keywords = Vec::new();

        if let Some(kc) = keyword_count {
            for _ in 0..kc {
                // It's actually only up to keyword count
                if let Ok(kwda) = KWDA::read(&mut cursor) {
                    keywords.push(FormID::read_le(&mut Cursor::new(kwda.data)).unwrap());
                }
            }
        }

        let data = DATA::read(&mut cursor)?;

        let short_name = match (ONAM::read(&mut cursor), raw.localized) {
            (Ok(f), true) => Some(LocalizedString::Localized(f.try_into()?)),
            (Ok(z), false) => Some(LocalizedString::ZString(z.try_into()?)),
            (Err(_), _) => None,
        };

        Ok(Self {
            header: raw.header,
            edid,
            bounds,
            full_name,
            model_filename,
            model_textures,
            icon,
            message_icon,
            destruction_data,
            pickup_sound,
            drop_sound,
            description,
            keywords,
            data,
            short_name,
        })
    }
}
