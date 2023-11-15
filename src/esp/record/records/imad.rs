use super::prelude::*;

/// Image Space Adapter
#[derive(Debug)]
pub struct IMAD {
    pub editor_id: EditorId,
    pub data_count: (),
    // TODO:
}

impl Record for IMAD {
    const TYPE: RecordType = RecordType::new(b"IMAD");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
