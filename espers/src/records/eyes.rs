use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, LocalizedString};
use crate::error::Error;
use crate::fields::{DATA, EDID, FULL, ICON};
use binrw::{binrw, BinRead};
use bitflags::bitflags;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"EYES")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EYES {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,

    #[br(calc(localized))]
    #[bw(ignore)]
    pub localized: bool,
}

bitflags! {
    #[binrw]
    #[brw(little)]
    #[derive(Deserialize, Serialize)]
    pub struct EyesFlags: u8 {
        const PLAYABLE = 0x01;
        const NOT_MALE = 0x02;
        const NOT_FEMALE = 0x04;
    }
}

impl TryFrom<DATA> for EyesFlags {
    type Error = Error;

    fn try_from(raw: DATA) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Eyes {
    pub header: RecordHeader,
    pub edid: Option<String>,
    pub full_name: Option<LocalizedString>,
    pub icon: Option<String>,
    pub flags: Option<EyesFlags>,
}

impl fmt::Display for Eyes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Eyes ({})",
            self.edid
                .as_ref()
                .map(|value| value.as_str())
                .unwrap_or_default()
        )
    }
}

impl TryFrom<EYES> for Eyes {
    type Error = Error;

    fn try_from(raw: EYES) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        let full_name = if raw.localized {
            FULL::read(&mut cursor)
                .ok()
                .map(|value| {
                    Ok::<LocalizedString, crate::error::Error>(LocalizedString::Localized(
                        value.try_into()?,
                    ))
                })
                .transpose()
        } else {
            FULL::read(&mut cursor)
                .ok()
                .map(|value| {
                    Ok::<LocalizedString, crate::error::Error>(LocalizedString::ZString(
                        value.try_into()?,
                    ))
                })
                .transpose()
        }?;
        let icon = ICON::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        let flags = DATA::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            full_name,
            icon,
            flags,
        })
    }
}
