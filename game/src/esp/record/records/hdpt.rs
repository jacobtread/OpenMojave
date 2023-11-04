use bitflags::bitflags;
use nom::{combinator::map, number::complete::u8, IResult};

use crate::esp::{
    record::{
        sub::{model::ModelData, DATA, EDID, FULL, HNAM},
        Collection, FromRecordBytes, Record, RecordParseError, RecordParser, RecordType,
    },
    shared::{EditorId, FormId},
};

/// Head part
pub struct HDPT {
    pub editor_id: EditorId,
    pub name: String,
    pub model_data: Option<ModelData>,
    pub flags: Flags,
    pub extra_parts: Vec<FormId>,
}

impl Record for HDPT {
    const TYPE: RecordType = RecordType::new(b"HDPT");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id = parser.parse::<EditorId>(EDID)?;
        let name = parser.parse::<String>(FULL)?;
        // TODO: Not sure if this field is optional documentation unclear
        let model_data = ModelData::parse_first(parser)?;
        let flags = parser.parse::<Flags>(DATA)?;
        let extra_parts = parser.parse::<Collection<FormId>>(HNAM)?.into_inner();

        Ok(Self {
            editor_id,
            name,
            model_data,
            flags,
            extra_parts,
        })
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct Flags: u8 {
        const PLAYABLE = 0x01;
    }
}

impl FromRecordBytes for Flags {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}
