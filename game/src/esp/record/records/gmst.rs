use crate::esp::{
    record::{
        sub::{DATA, EDID},
        Record, RecordParseError, RecordParser, RecordType,
    },
    shared::EditorId,
};

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
        let editor_id = parser.parse::<EditorId>(EDID)?;
        let first_char = editor_id
            .chars()
            .next()
            .ok_or_else(|| RecordParseError::Custom("Game setting editor ID was empty".into()))?;

        let value = match first_char {
            's' => parser.parse::<String>(DATA).map(GMSTValue::String)?,
            'f' => parser.parse::<f32>(DATA).map(GMSTValue::Float)?,
            // Default parsing as int
            _ => parser.parse::<i32>(DATA).map(GMSTValue::Int)?,
        };

        Ok(Self { editor_id, value })
    }
}
