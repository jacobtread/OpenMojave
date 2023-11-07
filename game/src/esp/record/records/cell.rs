use super::prelude::*;

/// Cell
#[derive(Debug)]
pub struct CELL {
    // TODO:
}

impl Record for CELL {
    const TYPE: RecordType = CELL;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
