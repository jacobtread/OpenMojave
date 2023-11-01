use crate::esp::records::record::{RecordType, SubRecord};
use nom::{combinator::map, number::complete::le_u32};

/// Script reference
#[derive(Debug)]
pub struct SCRO(pub u32);

impl SubRecord for SCRO {
    const TYPE: RecordType = RecordType::from_value(b"SCRO");

    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(le_u32, Self)(input)
    }
}
