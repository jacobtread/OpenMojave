use super::{
    idle::IDLE,
    prelude::{model::ModelData, *},
};

/// Animated Object
#[derive(Debug)]
pub struct ANIO {
    pub editor_id: EditorId,
    pub model_data: ModelData,
    pub data: TypedFormId<IDLE>,
}

impl Record for ANIO {
    const TYPE: RecordType = RecordType::new(b"ANIO");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let model_data: ModelData = ModelData::parse_first(parser)?
            .ok_or_else(|| RecordParseError::Custom("ANIO missing model data".to_string()))?;
        let data: TypedFormId<IDLE> = parser.parse(DATA)?;

        Ok(Self {
            editor_id,
            model_data,
            data,
        })
    }
}
