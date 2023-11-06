use super::prelude::*;

/// Projectile
#[derive(Debug)]
pub struct PROJ {}

impl Record for PROJ {
    const TYPE: RecordType = RecordType::new(b"PROJ");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
