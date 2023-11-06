use super::prelude::*;

#[derive(Debug)]
pub struct AMMO {
    // TODO:
}

impl Record for AMMO {
    const TYPE: RecordType = RecordType::new(b"AMMO");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
