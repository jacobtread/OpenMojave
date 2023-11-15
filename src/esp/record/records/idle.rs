use super::prelude::*;

/// Idle Animation
#[derive(Debug)]
pub struct IDLE {
    // TODO:
}

impl Record for IDLE {
    const TYPE: RecordType = RecordType::new(b"IDLE");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
