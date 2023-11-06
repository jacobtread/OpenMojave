use super::prelude::*;

/// Media Set
#[derive(Debug)]
pub struct MSET {}

impl Record for MSET {
    const TYPE: RecordType = RecordType::new(b"MSET");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
