use super::prelude::*;

/// Combat Style
#[derive(Debug)]
pub struct CSTY {
    // TODO:
}

impl Record for CSTY {
    const TYPE: RecordType = CSTY;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
