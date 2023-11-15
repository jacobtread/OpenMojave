use super::prelude::*;

/// Leveled NPC
#[derive(Debug)]
pub struct LVLN {}

impl Record for LVLN {
    const TYPE: RecordType = RecordType::new(b"LVLN");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
