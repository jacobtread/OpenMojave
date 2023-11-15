use super::prelude::*;

/// Leveled Creature
#[derive(Debug)]
pub struct LVLC {}

impl Record for LVLC {
    const TYPE: RecordType = RecordType::new(b"LVLC");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
