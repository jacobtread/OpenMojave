use crate::esp::records::record::{RecordType, SubRecord};
use nom::{
    character::complete::anychar,
    combinator::{map, rest},
};

/// Script Source
#[derive(Debug)]
pub struct SCTX(Vec<char>);

impl SubRecord for SCTX {
    const TYPE: RecordType = RecordType::from_value(b"SCTX");

    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        let chars: Vec<char> = input.iter().map(|value| *value as char).collect();
        Ok((&[], Self(chars)))
    }
}
