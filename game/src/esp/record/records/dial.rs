use super::prelude::*;

/// Dialog Topic
#[derive(Debug)]
pub struct DIAL {
    // TODO:
}

impl Record for DIAL {
    const TYPE: RecordType = DIAL;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
