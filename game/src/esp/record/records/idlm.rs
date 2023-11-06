use super::prelude::*;

/// Idle Marker
#[derive(Debug)]
pub struct IDLM {}

impl Record for IDLM {
    const TYPE: RecordType = RecordType::new(b"IDLM");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
