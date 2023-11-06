use super::prelude::*;

/// Navigation Mesh Info Map
#[derive(Debug)]
pub struct NAVI {}

impl Record for NAVI {
    const TYPE: RecordType = RecordType::new(b"NAVI");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
