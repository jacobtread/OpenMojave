use super::prelude::*;

/// Reputation
#[derive(Debug)]
pub struct REPU {
    pub editor_id: EditorId,
    pub name: Option<String>,
    pub large_icon_file_name: Option<String>,
    pub small_icon_file_name: Option<String>,
    pub data: Option<f32>,
}

impl Record for REPU {
    const TYPE: RecordType = RecordType::new(b"REPU");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let name: Option<String> = parser.try_parse(FULL)?;
        let large_icon_file_name: Option<String> = parser.try_parse(ICON)?;
        let small_icon_file_name: Option<String> = parser.try_parse(MICO)?;
        let data: Option<f32> = parser.try_parse(DATA)?;

        Ok(Self {
            editor_id,
            name,
            large_icon_file_name,
            small_icon_file_name,
            data,
        })
    }
}
