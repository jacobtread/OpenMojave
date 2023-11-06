use super::prelude::*;

/// Leveled Item
#[derive(Debug)]
pub struct LVLI {}

impl Record for LVLI {
    const TYPE: RecordType = RecordType::new(b"LVLI");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
