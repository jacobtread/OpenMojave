use crate::esp::{
    record::{
        sub::{script::Script, EDID},
        Record, RecordCollection, RecordParseError, RecordParser, RecordType,
    },
    shared::EditorId,
};

/// Script
#[derive(Debug)]
pub struct SCPT {
    pub editor_id: EditorId,
    pub script: Script,
}

impl Record for SCPT {
    const TYPE: RecordType = RecordType::from_value(b"SCPT");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id = parser.parse(EDID)?;
        let script = Script::parse_next(parser)?
            .ok_or_else(|| RecordParseError::Custom("SCPT missing script".to_string()))?;
        Ok(Self { editor_id, script })
    }
}