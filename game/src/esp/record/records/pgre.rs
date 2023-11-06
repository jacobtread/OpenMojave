use super::prelude::*;

/// Placed Grenade
#[derive(Debug)]
pub struct PGRE {}

impl Record for PGRE {
    const TYPE: RecordType = RecordType::new(b"CREA");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
