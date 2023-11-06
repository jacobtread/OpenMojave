use super::prelude::*;

/// Placed Missile
#[derive(Debug)]
pub struct PMIS {}

impl Record for PMIS {
    const TYPE: RecordType = RecordType::new(b"PMIS");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
