use super::prelude::*;

/// Ragdoll
#[derive(Debug)]
pub struct RGDL {}

impl Record for RGDL {
    const TYPE: RecordType = RecordType::new(b"RGDL");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
