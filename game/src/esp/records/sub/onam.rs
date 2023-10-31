use crate::esp::{
    records::record::{RecordType, SubRecord},
    shared::FormId,
};
use nom::{
    combinator::{all_consuming, map},
    multi::many0,
};

#[derive(Debug)]
pub struct ONAM(pub Vec<FormId>);

impl SubRecord for ONAM {
    const TYPE: RecordType = RecordType::from_value(b"ONAM");

    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        all_consuming(map(many0(FormId::parse), Self))(input)
    }
}
