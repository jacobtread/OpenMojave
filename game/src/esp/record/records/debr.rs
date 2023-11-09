use super::prelude::*;

/// Debris
#[derive(Debug)]
pub struct DEBR {
    // TODO:
}

impl Record for DEBR {
    const TYPE: RecordType = DEBR;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
