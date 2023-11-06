use super::prelude::*;

/// Game setting
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
    const TYPE: RecordType = RecordType::new(b"GMST");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id = parser.parse::<EditorId>(EDID)?;
        let first_char = editor_id
            .chars()
            .next()
            .ok_or_else(|| RecordParseError::Custom("Game setting editor ID was empty".into()))?;

        let value = match first_char {
            's' => parser.parse(DATA).map(GMSTValue::String)?,
            'f' => parser.parse(DATA).map(GMSTValue::Float)?,
            // Default parsing as int
            _ => parser.parse(DATA).map(GMSTValue::Int)?,
        };

        Ok(Self { editor_id, value })
    }
}
