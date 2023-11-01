use crate::esp::shared::FormId;

use super::{
    parse_cstring,
    record::{nom_prelude::*, Record, RecordParseError, RecordParser, RecordType},
    sub::{CNAM, DATA, SNAM},
};
use nom::{number::complete::le_f32, sequence::tuple};

#[derive(Debug)]
pub struct TES4 {
    pub hedr: HEDR,
    pub author: String,
    pub description: Option<String>,
    pub masters: Vec<String>,
    pub form_overrides: Option<Vec<FormId>>,
}

impl Record for TES4 {
    const TYPE: RecordType = RecordType::from_value(b"TES4");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let hedr = parser.parse(HEDR::TYPE, HEDR::parse)?;

        parser.skip_type(RecordType::from_value(b"OFST"));
        parser.skip_type(RecordType::from_value(b"DELE"));

        let author: String = parser.parse(CNAM::TYPE, CNAM::parse_string)?;
        let description = parser.try_parse(SNAM::TYPE, SNAM::parse_string)?;

        let mut masters: Vec<String> = Vec::new();

        // Consume master data collection
        while let Some(mast) = parser.try_parse(MAST::TYPE, MAST::parse)? {
            // Data can be ignored as its not used
            parser.skip_type(DATA::TYPE);
            masters.push(mast);
        }

        let form_overrides = parser.try_parse(ONAM::TYPE, ONAM::parse)?;

        parser.skip_type(RecordType::from_value(b"DELE"));

        Ok(Self {
            hedr,
            author,
            description,
            masters,
            form_overrides,
        })
    }
}

/// "HEDR" sub record
#[derive(Debug)]
pub struct HEDR {
    /// 0.94 in most files; 1.7 in recent versions of Update.esm.
    pub version: f32,
    /// Number of records and groups (not including TES4 record itself).
    pub num_records: u32,
    /// Next available object ID.
    pub next_object_id: u32,
}

impl HEDR {
    const TYPE: RecordType = RecordType::from_value(b"HEDR");

    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((le_f32, le_u32, le_u32)),
            |(version, num_records, next_object_id)| HEDR {
                version,
                num_records,
                next_object_id,
            },
        )(input)
    }
}

pub struct MAST;

impl MAST {
    pub const TYPE: RecordType = RecordType::from_value(b"MAST");

    pub fn parse(input: &[u8]) -> IResult<&[u8], String> {
        parse_cstring(input)
    }
}

pub struct ONAM;

impl ONAM {
    pub const TYPE: RecordType = RecordType::from_value(b"ONAM");

    pub fn parse(input: &[u8]) -> IResult<&[u8], Vec<FormId>> {
        many0(FormId::parse)(input)
    }
}
