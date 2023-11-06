use super::prelude::*;

/// Combat Style
#[derive(Debug)]
pub struct CSTY {
    // TODO:
}

impl Record for CSTY {
    const TYPE: RecordType = RecordType::new(b"CSTY");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
