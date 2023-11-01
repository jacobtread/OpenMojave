use crate::esp::records::{
    parse_cstring,
    record::{RecordType, SubRecord},
};
use nom::combinator::map;

#[derive(Debug)]
pub struct SNAM(pub String);

impl SubRecord for SNAM {
    const TYPE: RecordType = RecordType::from_value(b"SNAM");

    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(parse_cstring, Self)(input)
    }
}
