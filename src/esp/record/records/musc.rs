use super::prelude::*;

/// Music Type
#[derive(Debug)]
pub struct MUSC {
    pub editor_id: EditorId,
    pub file_name: Option<String>,
    pub db: Option<f32>,
}

impl Record for MUSC {
    const TYPE: RecordType = RecordType::new(b"MUSC");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let file_name: Option<String> = parser.try_parse(FNAM)?;
        let db: Option<f32> = parser.try_parse(ANAM)?;

        Ok(Self {
            editor_id,
            file_name,
            db,
        })
    }
}
