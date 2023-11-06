use super::prelude::*;

/// Explosion
#[derive(Debug)]
pub struct EXPL {
    // TODO:
}

impl Record for EXPL {
    const TYPE: RecordType = RecordType::new(b"EXPL");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
