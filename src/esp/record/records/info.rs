use super::prelude::*;

/// Dialog Response
#[derive(Debug)]
pub struct INFO {
    // TODO:
}

impl Record for INFO {
    const TYPE: RecordType = RecordType::new(b"INFO");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
