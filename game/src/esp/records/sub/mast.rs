use nom::combinator::map;

use crate::esp::records::{
    parse_cstring,
    record::{RecordType, SubRecord},
};

#[derive(Debug)]
pub struct MAST(pub String);

impl SubRecord for MAST {
    const TYPE: RecordType = RecordType::from_value(b"MAST");

    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(parse_cstring, Self)(input)
    }
}
