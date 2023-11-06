use super::prelude::*;

/// Landscape
#[derive(Debug)]
pub struct LAND {
    // TODO:
}

impl Record for LAND {
    const TYPE: RecordType = RecordType::new(b"LAND");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
