use super::prelude::*;

/// Quest
#[derive(Debug)]
pub struct QUST {}

impl Record for QUST {
    const TYPE: RecordType = RecordType::new(b"QUST");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
