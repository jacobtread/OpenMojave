use bitflags::bitflags;
use nom::{combinator::map, number::complete::u8, IResult};

use crate::esp::{
    record::{Record, RecordParseError, RecordParser, RecordType},
    shared::{EditorId, FormId},
};

/// Head part
pub struct HDPT {
    pub editor_id: EditorId,
    pub name: String,
    pub model_data: (),
    pub data: Flags,
    pub extra_parts: Vec<FormId>,
}

impl Record for HDPT {
    const TYPE: RecordType = RecordType::from_value(b"HDPT");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct Flags: u8 {
        const PLAYABLE = 0x01;
    }
}

impl Flags {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(u8, Flags::from_bits_retain)(input)
    }
}
