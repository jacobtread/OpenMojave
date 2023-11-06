use super::prelude::*;

/// Image Space
#[derive(Debug)]
pub struct IMGS {
    pub editor_id: EditorId,
    pub dnam: (),
    // TODO:
}

impl Record for IMGS {
    const TYPE: RecordType = RecordType::new(b"IMGS");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
