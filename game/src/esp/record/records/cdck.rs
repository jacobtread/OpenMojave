use super::{ccrd::CCRD, prelude::*};

/// Caravan Deck
#[derive(Debug)]
pub struct CDCK {
    pub editor_id: EditorId,
    pub name: Option<String>,
    pub cards: Vec<TypedFormId<CCRD>>,
    pub data: Option<u32>,
}

impl Record for CDCK {
    const TYPE: RecordType = RecordType::new(b"CDCK");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let name: Option<String> = parser.try_parse(FULL)?;
        let cards: Vec<TypedFormId<CCRD>> = parser.try_parse_many(CARD)?;
        let data: Option<u32> = parser.try_parse(DATA)?;

        Ok(Self {
            editor_id,
            name,
            cards,
            data,
        })
    }
}
