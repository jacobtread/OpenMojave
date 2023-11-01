use crate::esp::records::{
    parse_cstring,
    record::{RecordType, SubRecord},
};
use nom::combinator::map;

// Local variable name
#[derive(Debug)]
pub struct SCVR(pub String);

impl SubRecord for SCVR {
    const TYPE: RecordType = RecordType::from_value(b"SCVR");

    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(parse_cstring, Self)(input)
    }
}
