use super::prelude::*;

/// Placed Object
#[derive(Debug)]
pub struct REFR {}

impl Record for REFR {
    const TYPE: RecordType = RecordType::new(b"REFR");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
