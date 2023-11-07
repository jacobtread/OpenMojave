use super::prelude::*;

/// Body Part Data
#[derive(Debug)]
pub struct BPTD {
    // TODO:
}

impl Record for BPTD {
    const TYPE: RecordType = BPTD;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
