use bitflags::bitflags;
use nom::{combinator::map, number::complete::u8};

use crate::esp::{
    record::{
        sub::{model::ModelData, DATA, EDID, FULL, ICON},
        FromRecordBytes, Record, RecordParseError, RecordParser, RecordType,
    },
    shared::EditorId,
};

#[derive(Debug)]
pub struct HAIR {
    pub editor_id: EditorId,
    pub name: String,
    pub model_data: ModelData,
    pub texture: String,
    pub flags: Flags,
}

impl Record for HAIR {
    const TYPE: RecordType = RecordType::from_value(b"HAIR");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id = parser.parse::<EditorId>(EDID)?;
        let name = parser.parse::<String>(FULL)?;
        let model_data = ModelData::parse_first(parser)?
            .ok_or_else(|| RecordParseError::Custom("Hair missing model data".to_string()))?;
        let texture = parser.parse::<String>(ICON)?;
        let flags = parser.parse::<Flags>(DATA)?;
        Ok(Self {
            editor_id,
            name,
            model_data,
            texture,
            flags,
        })
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct Flags: u8 {
        const PLAYABLE   = 0x01;
        const NOT_MALE   = 0x02;
        const NOT_FEMALE = 0x04;
        const FIXED      = 0x08;
    }
}

impl FromRecordBytes for Flags {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}
