use super::prelude::*;

/// Media Relation Controller
#[derive(Debug)]
pub struct ALOC {
    // TODO:
}

impl Record for ALOC {
    const TYPE: RecordType = RecordType::new(b"ALOC");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
