use super::prelude::*;

/// Challenge
#[derive(Debug)]
pub struct CHAL {
    // TODO:
}

impl Record for CHAL {
    const TYPE: RecordType = RecordType::new(b"CHAL");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
