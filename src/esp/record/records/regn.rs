use super::prelude::*;

/// Region
#[derive(Debug)]
pub struct REGN {}

impl Record for REGN {
    const TYPE: RecordType = RecordType::new(b"REGN");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
