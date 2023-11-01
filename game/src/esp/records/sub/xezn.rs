use crate::esp::{
    records::record::{RecordType, SubRecord},
    shared::FormId,
};
use nom::combinator::map;

#[derive(Debug)]
pub struct XEZN(pub FormId);

impl SubRecord for XEZN {
    const TYPE: RecordType = RecordType::from_value(b"XEZN");

    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(FormId::parse, Self)(input)
    }
}
