use crate::esp::records::record::{RecordType, SubRecord};
use nom::{
    combinator::{all_consuming, map, rest},
    number::complete::le_f32,
    Parser,
};

/// Idle Time
#[derive(Debug)]
pub struct XPRD(f32);

impl SubRecord for XPRD {
    const TYPE: RecordType = RecordType::from_value(b"XPRD");

    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(le_f32, Self)(input)
    }
}
