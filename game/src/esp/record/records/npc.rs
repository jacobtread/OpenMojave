use super::prelude::*;

/// Non-Player Character
#[derive(Debug)]
pub struct NPC {}

impl Record for NPC {
    const TYPE: RecordType = RecordType::new(b"NPC_");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
