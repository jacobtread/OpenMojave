use super::prelude::*;

/// Dehydration Stage
#[derive(Debug)]
pub struct DEHY {
    // TODO:
}

impl Record for DEHY {
    const TYPE: RecordType = RecordType::new(b"DEHY");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
