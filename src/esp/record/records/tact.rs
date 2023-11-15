use super::{prelude::*, scpt::SCPT, soun::SOUN, vtyp::VTYP};
use crate::esp::record::sub::{
    destruction::DestructionData, model::ModelData, object_bounds::ObjectBounds,
};

/// Talking Activator
#[derive(Debug)]
pub struct TACT {
    pub editor_id: EditorId,
    pub object_bounds: ObjectBounds,
    pub name: Option<String>,
    pub model_data: ModelData,
    pub script: Option<TypedFormId<SCPT>>,
    pub destruction_data: Option<DestructionData>,
    pub sound_looping: Option<TypedFormId<SOUN>>,
    pub voice_type: Option<TypedFormId<VTYP>>,
    pub radio_template: Option<TypedFormId<SOUN>>,
}

impl Record for TACT {
    const TYPE: RecordType = RecordType::new(b"TACT");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let object_bounds: ObjectBounds = parser.parse(OBND)?;
        let name: Option<String> = parser.try_parse(FULL)?;
        let model_data: ModelData = ModelData::parse_first(parser)?
            .ok_or_else(|| RecordParseError::Custom("Missing model data for TACT".to_string()))?;
        let script: Option<TypedFormId<SCPT>> = parser.try_parse(SCRI)?;
        let destruction_data: Option<DestructionData> = DestructionData::parse_next(parser)?;
        let sound_looping: Option<TypedFormId<SOUN>> = parser.try_parse(SNAM)?;
        let voice_type: Option<TypedFormId<VTYP>> = parser.try_parse(VNAM)?;
        let radio_template: Option<TypedFormId<SOUN>> = parser.try_parse(INAM)?;

        Ok(Self {
            editor_id,
            object_bounds,
            name,
            model_data,
            script,
            destruction_data,
            sound_looping,
            voice_type,
            radio_template,
        })
    }
}
