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
    const TYPE: RecordType = RecordType::new(b"SCPT");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let script: Script = Script::parse_next(parser)?
            .ok_or_else(|| RecordParseError::Custom("SCPT missing script".to_string()))?;
        Ok(Self { editor_id, script })
    }
}
