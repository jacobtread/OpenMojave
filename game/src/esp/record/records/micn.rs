use super::prelude::*;

/// Menu icon
#[derive(Debug)]
pub struct MICN {
    pub editor_id: EditorId,
    pub large_icon_file_name: String,
    pub small_icon_file_name: String,
}

impl Record for MICN {
    const TYPE: RecordType = RecordType::new(b"MICN");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let large_icon_file_name: String = parser.parse(ICON)?;
        let small_icon_file_name: String = parser.parse(MICO)?;

        Ok(Self {
            editor_id,
            large_icon_file_name,
            small_icon_file_name,
        })
    }
}
