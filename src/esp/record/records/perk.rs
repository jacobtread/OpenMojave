use super::prelude::*;

/// Perk
#[derive(Debug)]
pub struct PERK {}

impl Record for PERK {
    const TYPE: RecordType = RecordType::new(b"PERK");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
