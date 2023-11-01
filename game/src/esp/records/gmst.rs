use nom::{
    combinator::map,
    number::complete::{le_f32, le_i32},
};

use crate::esp::{
    records::{
        parse_cstring,
        sub::{DATA, EDID},
    },
    shared::EditorId,
};

use super::record::{Record, RecordParseError, RecordParser, RecordType};

#[derive(Debug)]
pub struct GMST {
    pub editor_id: EditorId,
    pub value: GMSTValue,
}

#[derive(Debug)]
pub enum GMSTValue {
    String(String),
    Float(f32),
    Int(i32),
}

impl Record for GMST {
    const TYPE: RecordType = RecordType::from_value(b"GMST");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id = parser.parse(EDID::TYPE, EDID::parse)?;
        let first_char = editor_id
            .chars()
            .next()
            .ok_or_else(|| RecordParseError::Custom("Game setting editor ID was empty".into()))?;

        let value = match first_char {
            's' => parser.parse(DATA::TYPE, map(parse_cstring, GMSTValue::String))?,
            'f' => parser.parse(DATA::TYPE, map(le_f32, GMSTValue::Float))?,
            // Default parsing as int
            _ => parser.parse(DATA::TYPE, map(le_i32, GMSTValue::Int))?,
        };

        Ok(Self { editor_id, value })
    }
}
