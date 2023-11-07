use super::prelude::*;

/// Actor Value Information
#[derive(Debug)]
pub struct AVIF {
    pub editor_id: EditorId,
    pub name: Option<String>,
    pub description: String,
    pub large_icon_file_name: Option<String>,
    pub small_icon_file_name: Option<String>,
    pub short_name: Option<String>,
}

impl Record for AVIF {
    const TYPE: RecordType = AVIF;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let name: Option<String> = parser.try_parse(FULL)?;
        let description: String = parser.parse(DESC)?;
        let large_icon_file_name: Option<String> = parser.try_parse(ICON)?;
        let small_icon_file_name: Option<String> = parser.try_parse(MICO)?;
        let short_name: Option<String> = parser.try_parse(ANAM)?;

        Ok(Self {
            editor_id,
            name,
            description,
            large_icon_file_name,
            small_icon_file_name,
            short_name,
        })
    }
}
