use super::prelude::*;
use crate::esp::record::sub::script::Script;

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
        let script: Script = Script::require_parse_next(parser)?;
        Ok(Self { editor_id, script })
    }
}
