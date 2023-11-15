use super::prelude::*;

/// Creature
#[derive(Debug)]
pub struct CREA {
    // TODO:
}

impl Record for CREA {
    const TYPE: RecordType = CREA;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
