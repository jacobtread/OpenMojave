use super::prelude::*;

/// FormID List
#[derive(Debug)]
pub struct FLST {
    pub editor_id: EditorId,
    pub form_ids: Vec<FormId>,
}

impl Record for FLST {
    const TYPE: RecordType = RecordType::new(b"FLST");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let form_ids: Vec<FormId> = parser.try_parse_many(LNAM)?;

        Ok(Self {
            editor_id,
            form_ids,
        })
    }
}
