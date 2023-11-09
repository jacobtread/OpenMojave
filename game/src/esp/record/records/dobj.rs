use super::prelude::*;

/// Default Object Manager
#[derive(Debug)]
pub struct DOBJ {
    // TODO:
}

impl Record for DOBJ {
    const TYPE: RecordType = DOBJ;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
