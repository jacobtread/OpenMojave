use super::{prelude::*, scpt::SCPT};
use crate::esp::record::sub::{
    destruction::DestructionData, model::ModelData, object_bounds::ObjectBounds,
};

/// Furniture
#[derive(Debug)]
pub struct FURN {
    pub editor_id: EditorId,
    pub object_bounds: ObjectBounds,
    pub name: Option<String>,
    pub model_data: ModelData,
    pub script: Option<TypedFormId<SCPT>>,
    pub destruction_data: Option<DestructionData>,
    pub marker_flags: Vec<u8>,
}

impl Record for FURN {
    const TYPE: RecordType = RecordType::new(b"FURN");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let object_bounds: ObjectBounds = parser.parse(OBND)?;
        let name: Option<String> = parser.try_parse(FULL)?;
        let model_data: ModelData = ModelData::parse_first(parser)?
            .ok_or_else(|| RecordParseError::Custom("FURN missing model data".to_string()))?;
        let script: Option<TypedFormId<SCPT>> = parser.try_parse(SCRI)?;
        let destruction_data: Option<DestructionData> = DestructionData::parse_next(parser)?;
        let marker_flags: Vec<u8> = parser.parse::<Repeated<u8>>(MNAM)?.into_inner();

        Ok(Self {
            editor_id,
            object_bounds,
            name,
            model_data,
            script,
            destruction_data,
            marker_flags,
        })
    }
}
