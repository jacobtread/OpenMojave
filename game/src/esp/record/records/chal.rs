use super::prelude::*;

/// Challenge
#[derive(Debug)]
pub struct CHAL {
    // TODO:
}

impl Record for CHAL {
    const TYPE: RecordType = CHAL;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
