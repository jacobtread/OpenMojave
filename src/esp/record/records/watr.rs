use super::prelude::*;

#[derive(Debug)]
pub struct WATR {}

impl Record for WATR {
    const TYPE: RecordType = RecordType::new(b"WATR");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
