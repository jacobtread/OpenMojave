use super::prelude::*;

/// Casino
#[derive(Debug)]
pub struct CSNO {}

impl Record for CSNO {
    const TYPE: RecordType = CSNO;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
