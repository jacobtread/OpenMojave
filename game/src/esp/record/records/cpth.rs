use super::prelude::*;

#[derive(Debug)]
pub struct CPTH {
    // TODO:
}

impl Record for CPTH {
    const TYPE: RecordType = RecordType::new(b"CPTH");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
