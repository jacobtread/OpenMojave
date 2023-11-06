use super::prelude::*;

/// Effect Shader
#[derive(Debug)]
pub struct EFSH {
    // TODO:
}

impl Record for EFSH {
    const TYPE: RecordType = RecordType::new(b"EFSH");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
