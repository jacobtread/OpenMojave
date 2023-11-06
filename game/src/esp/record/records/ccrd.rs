use super::prelude::*;

#[derive(Debug)]
pub struct CCRD {
    // TODO:
}

impl Record for CCRD {
    const TYPE: RecordType = RecordType::new(b"CCRD");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
