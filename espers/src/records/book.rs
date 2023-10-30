use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID, LocalizedString};
use crate::error::Error;
use crate::fields::{
    DestructionData, Model, ObjectBounds, ScriptList, CNAM, DATA, DESC, EDID, FULL, ICON, INAM,
    KSIZ, KWDA, MICO, OBND, VMAD, YNAM, ZNAM,
};
use crate::string_table::StringTables;
use binrw::{binrw, BinRead};
use bitflags::bitflags;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

bitflags! {
    #[binrw]
    #[brw(little)]
    #[derive(Deserialize, Serialize)]
    pub struct BookFlags: u8 {
        const TEACHES_SKILL = 0x01;
        const CANNOT_BE_TAKEN = 0x02;
        const TEACHES_SPELL = 0x04;
        const READ = 0x08;
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookData {
    pub flags: BookFlags,
    pub kind: u8,
    // pub unknown: [u8; 2],
    // pub teaches: FormID,
    pub value: u32,
    pub weight: f32,
}

impl TryFrom<DATA> for BookData {
    type Error = Error;

    fn try_from(raw: DATA) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"BOOK")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BOOK {
    pub header: RecordHeader,
    #[br(count = header.size)]
    pub data: Vec<u8>,

    #[br(calc(localized))]
    #[bw(ignore)]
    pub localized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub header: RecordHeader,
    pub edid: String,
    pub scripts: Option<ScriptList>,
    pub bounds: ObjectBounds,
    pub full_name: Option<LocalizedString>,
    pub model: Option<Model>,
    pub inventory_image: Option<String>,
    pub message_image: Option<String>,
    pub text: LocalizedString,
    pub destruction_data: Option<DestructionData>,
    pub pickup_sound: Option<FormID>,
    pub drop_sound: Option<FormID>,
    pub keywords: Vec<FormID>,
    pub data: BookData,
    pub inventory_art: Option<FormID>,
    pub description: LocalizedString,
}

impl Book {
    pub fn localize(&mut self, string_table: &StringTables) {
        if let Some(LocalizedString::Localized(l)) = self.full_name {
            if let Some(s) = string_table.get_string(&l) {
                self.full_name = Some(LocalizedString::ZString(s.clone()));
            }
        }

        if let LocalizedString::Localized(l) = self.text {
            if let Some(s) = string_table.get_string(&l) {
                self.text = LocalizedString::ZString(s.clone());
            }
        }

        if let LocalizedString::Localized(l) = self.description {
            if let Some(s) = string_table.get_string(&l) {
                self.description = LocalizedString::ZString(s.clone());
            }
        }
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Book ({})", self.edid)
    }
}

impl TryFrom<BOOK> for Book {
    type Error = Error;

    fn try_from(raw: BOOK) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let scripts = VMAD::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let bounds = OBND::read(&mut cursor)?.try_into()?;
        let full_name = match (FULL::read(&mut cursor), raw.localized) {
            (Ok(f), true) => Some(LocalizedString::Localized(f.try_into()?)),
            (Ok(z), false) => Some(LocalizedString::ZString(z.try_into()?)),
            (Err(_), _) => None,
        };
        let model = Model::try_load(&mut cursor, raw.header.internal_version)?;
        let inventory_image = ICON::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let message_image = MICO::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let text = if raw.localized {
            LocalizedString::Localized(DESC::read(&mut cursor)?.try_into()?)
        } else {
            LocalizedString::ZString(DESC::read(&mut cursor)?.try_into()?)
        };
        let destruction_data = DestructionData::load(&mut cursor)?;
        let pickup_sound = YNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let drop_sound = ZNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

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

        let data = DATA::read(&mut cursor)?.try_into()?;
        let inventory_art = INAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        let description = if raw.localized {
            LocalizedString::Localized(CNAM::read(&mut cursor)?.try_into()?)
        } else {
            LocalizedString::ZString(CNAM::read(&mut cursor)?.try_into()?)
        };

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            scripts,
            bounds,
            full_name,
            model,
            inventory_image,
            message_image,
            text,
            destruction_data,
            pickup_sound,
            drop_sound,
            keywords,
            data,
            inventory_art,
            description,
        })
    }
}
