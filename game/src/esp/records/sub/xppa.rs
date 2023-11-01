use crate::esp::records::record::{RecordType, SubRecord};
use nom::combinator::{map, rest_len, verify};

/// Patrol script marker
#[derive(Debug)]
pub struct XPPA;

impl SubRecord for XPPA {
    const TYPE: RecordType = RecordType::from_value(b"XPPA");

    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        // XPPA should be empty
        map(verify(rest_len, |len| 0.eq(len)), |_| Self)(input)
    }
}
