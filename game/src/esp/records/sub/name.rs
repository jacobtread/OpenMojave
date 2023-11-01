use crate::esp::{
    records::record::{RecordType, SubRecord},
    shared::FormId,
};
use nom::combinator::map;

#[derive(Debug)]
pub struct NAME(pub FormId);

impl SubRecord for NAME {
    const TYPE: RecordType = RecordType::from_value(b"NAME");

    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(FormId::parse, Self)(input)
    }
}
