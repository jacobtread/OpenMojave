use super::prelude::*;

/// Navigation Mesh
#[derive(Debug)]
pub struct NAVM {}

impl Record for NAVM {
    const TYPE: RecordType = RecordType::new(b"CREA");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
