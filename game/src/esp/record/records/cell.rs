use super::prelude::*;

#[derive(Debug)]
pub struct CELL {
    // TODO:
}

impl Record for CELL {
    const TYPE: RecordType = RecordType::new(b"CELL");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
