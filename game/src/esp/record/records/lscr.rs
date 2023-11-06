use super::prelude::*;

/// Load Screen
#[derive(Debug)]
pub struct LSCR {}

impl Record for LSCR {
    const TYPE: RecordType = RecordType::new(b"LSCR");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
