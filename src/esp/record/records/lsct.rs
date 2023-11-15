use super::prelude::*;

/// Load Screen Type
#[derive(Debug)]
pub struct LSCT {}

impl Record for LSCT {
    const TYPE: RecordType = RecordType::new(b"LSCT");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
