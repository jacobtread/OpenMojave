use super::prelude::*;

#[derive(Debug)]
pub struct CAMS {
    // TODO:
}

impl Record for CAMS {
    const TYPE: RecordType = RecordType::new(b"CAMS");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
