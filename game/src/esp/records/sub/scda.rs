use crate::esp::records::record::{RecordType, SubRecord};
use nom::combinator::{map, rest};

/// Compiled Script Source
#[derive(Debug)]
pub struct SCDA(Vec<u8>);

impl SubRecord for SCDA {
    const TYPE: RecordType = RecordType::from_value(b"SCDA");

    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(rest, |rest: &[u8]| SCDA(rest.to_vec()))(input)
    }
}
