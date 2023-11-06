use super::prelude::*;

/// Climate
#[derive(Debug)]
pub struct CLMT {
    // TODO:
}

impl Record for CLMT {
    const TYPE: RecordType = RecordType::new(b"CLMT");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
