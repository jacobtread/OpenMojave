use super::prelude::*;

/// Armor Addon
#[derive(Debug)]
pub struct ARMA {
    // TODO:
}

impl Record for ARMA {
    const TYPE: RecordType = RecordType::new(b"ARMA");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
