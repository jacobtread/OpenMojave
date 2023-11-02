use crate::esp::{
    record::{
        sub::{EDID, ICON, MICO},
        Record, RecordParseError, RecordParser, RecordType,
    },
    shared::EditorId,
};

#[derive(Debug)]
pub struct MICN {
    pub editor_id: EditorId,
    pub large_icon_file_name: String,
    pub small_icon_file_name: String,
}

impl Record for MICN {
    const TYPE: RecordType = RecordType::from_value(b"MICN");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id = parser.parse::<EditorId>(EDID)?;
        let large_icon_file_name = parser.parse::<String>(ICON)?;
        let small_icon_file_name = parser.parse::<String>(MICO)?;

        Ok(Self {
            editor_id,
            large_icon_file_name,
            small_icon_file_name,
        })
    }
}
