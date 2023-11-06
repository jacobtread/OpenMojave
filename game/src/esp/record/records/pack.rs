use super::prelude::*;

/// Package (AI Package?)
#[derive(Debug)]
pub struct PACK {}

impl Record for PACK {
    const TYPE: RecordType = RecordType::new(b"PACK");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
