use super::{prelude::*, scpt::SCPT, soun::SOUN, tact::TACT, watr::WATR};
use crate::esp::record::sub::{
    destruction::DestructionData, model::ModelData, object_bounds::ObjectBounds,
};

/// Activator
#[derive(Debug)]
pub struct ACTI {
    pub editor_id: EditorId,
    pub object_bounds: ObjectBounds,
    pub name: Option<String>,
    pub model_data: Option<ModelData>,
    pub script: Option<TypedFormId<SCPT>>,
    pub destruction_data: Option<DestructionData>,
    pub sound_looping: Option<TypedFormId<SOUN>>,
    pub sound_activation: Option<TypedFormId<SOUN>>,
    pub radio_template: Option<TypedFormId<SOUN>>,
    pub radio_station: Option<TypedFormId<TACT>>,
    pub water_type: Option<TypedFormId<WATR>>,
    pub activation_prompt: Option<String>,
}

impl Record for ACTI {
    const TYPE: RecordType = ACTI;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let object_bounds: ObjectBounds = parser.parse(OBND)?;
        let name: Option<String> = parser.try_parse(FULL)?;
        let model_data: Option<ModelData> = ModelData::parse_first(parser)?;
        let script: Option<TypedFormId<SCPT>> = parser.try_parse(SCRI)?;
        let destruction_data: Option<DestructionData> = DestructionData::parse_next(parser)?;
        let sound_looping: Option<TypedFormId<SOUN>> = parser.try_parse(SNAM)?;
        let sound_activation: Option<TypedFormId<SOUN>> = parser.try_parse(VNAM)?;
        let radio_template: Option<TypedFormId<SOUN>> = parser.try_parse(INAM)?;
        let radio_station: Option<TypedFormId<TACT>> = parser.try_parse(RNAM)?;
        let water_type: Option<TypedFormId<WATR>> = parser.try_parse(WNAM)?;
        let activation_prompt: Option<String> = parser.try_parse(XATO)?;

        Ok(Self {
            editor_id,
            object_bounds,
            name,
            model_data,
            script,
            destruction_data,
            sound_looping,
            sound_activation,
            radio_template,
            radio_station,
            water_type,
            activation_prompt,
        })
    }
}
