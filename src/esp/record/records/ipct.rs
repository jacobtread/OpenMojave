use super::prelude::*;

/// Impact
#[derive(Debug)]
pub struct IPCT {
    // TODO:
}

impl Record for IPCT {
    const TYPE: RecordType = RecordType::new(b"IPCT");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
