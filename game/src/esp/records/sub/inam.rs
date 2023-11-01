use crate::esp::{
    records::record::{RecordType, SubRecord},
    shared::FormId,
};
use nom::combinator::map;

#[derive(Debug)]
pub struct INAM(pub FormId);

impl SubRecord for INAM {
    const TYPE: RecordType = RecordType::from_value(b"INAM");

    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(FormId::parse, Self)(input)
    }
}
