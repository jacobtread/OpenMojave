use crate::common::FormID;
use crate::error::Error;
use crate::fields::{CNAM, DATA, HEDR, INCC, INTV, MAST, ONAM, SNAM};
use binrw::{binrw, io::Cursor, BinRead, BinWrite};
use bitflags::bitflags;
use serde_derive::{Deserialize, Serialize};

bitflags! {
    #[binrw]
    #[derive(Serialize, Deserialize)]
    pub struct Flags: u32 {
        const MASTER = 0x00000001;
        const LOCALIZED = 0x00000080;
        const LIGHT_MASTER = 0x00000200;
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordHeader {
    pub size: u32,
    pub flags: Flags,
    pub form_id: u32,
    pub timestamp: u16,
    pub version_control: u16,
    pub internal_version: u16,
    pub unknown: u16,
}

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"TES4")]
pub struct TES4 {
    pub header: RecordHeader,
    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Header {
    pub header: RecordHeader,
    pub hedr: HEDR,
    pub author: Option<String>,
    pub description: Option<String>,
    pub masters: Vec<String>,
    pub overrides: Vec<FormID>,
    pub tagifiable_strings_count: u32,
    pub counter: Option<u32>,
}

impl TryFrom<TES4> for Header {
    type Error = Error;

    fn try_from(raw: TES4) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);

        let header = HEDR::read(&mut cursor)?;
        let author = CNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let description = SNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        let mut masters = Vec::new();

        loop {
            match MAST::read(&mut cursor) {
                Ok(m) => masters.push(m.try_into()?),
                Err(_) => break,
            }
            match DATA::read(&mut cursor) {
                Ok(_) => {}
                Err(_) => break,
            }
        }

        let overrides = ONAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?
            .unwrap_or_default();

        // let tagifiable_strings_count = INTV::read(&mut cursor)
        //     //
        //     .unwrap()
        //     .try_into()
        //     //
        //     .unwrap();
        let tagifiable_strings_count = 0;
        let counter = INCC::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()
            .unwrap();

        Ok(Self {
            header: raw.header,
            hedr: header,
            author,
            description,
            masters,
            overrides,
            tagifiable_strings_count,
            counter,
        })
    }
}

impl TryFrom<Header> for TES4 {
    type Error = Error;

    fn try_from(obj: Header) -> Result<Self, Self::Error> {
        let mut data = Cursor::new(Vec::new());
        obj.hedr.write(&mut data)?;

        if let Some(a) = obj.author {
            CNAM::try_from(a)?.write(&mut data)?;
        }

        if let Some(d) = obj.description {
            SNAM::try_from(d)?.write(&mut data)?;
        }

        for master in obj.masters {
            MAST::try_from(master)?.write(&mut data)?;
            DATA::try_from(vec![0u8; 8])?.write(&mut data)?;
        }

        if !obj.overrides.is_empty() {
            ONAM::try_from(obj.overrides)?.write(&mut data)?;
        }

        INTV::try_from(obj.tagifiable_strings_count)?.write(&mut data)?;

        if let Some(c) = obj.counter {
            INCC::try_from(c)?.write(&mut data)?;
        }
        Ok(Self {
            header: obj.header,
            data: data.into_inner(),
        })
    }
}
