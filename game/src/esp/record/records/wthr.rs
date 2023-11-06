use super::prelude::*;

/// Weather
#[derive(Debug)]
pub struct WTHR {
    // TODO:
}

impl Record for WTHR {
    const TYPE: RecordType = RecordType::new(b"WTHR");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
