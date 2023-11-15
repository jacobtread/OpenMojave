use super::{
    prelude::{destruction::DestructionData, model::ModelData, object_bounds::ObjectBounds, *},
    soun::SOUN,
};

/// Casino Chip
#[derive(Debug)]
pub struct CHIP {
    pub editor_id: EditorId,
    pub object_bounds: ObjectBounds,
    pub name: Option<String>,
    pub model_data: Option<ModelData>,
    pub large_icon_file_name: Option<String>,
    pub small_icon_file_name: Option<String>,
    pub destruction_data: Option<DestructionData>,
    pub sound_pick_up: Option<TypedFormId<SOUN>>,
    pub sound_drop: Option<TypedFormId<SOUN>>,
}

impl Record for CHIP {
    const TYPE: RecordType = CHIP;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let object_bounds: ObjectBounds = parser.parse(OBND)?;
        let name: Option<String> = parser.try_parse(FULL)?;
        let model_data: Option<ModelData> = ModelData::parse_first(parser)?;
        let large_icon_file_name: Option<String> = parser.try_parse(ICON)?;
        let small_icon_file_name: Option<String> = parser.try_parse(MICO)?;
        let destruction_data: Option<DestructionData> = DestructionData::parse_next(parser)?;
        let sound_pick_up: Option<TypedFormId<SOUN>> = parser.try_parse(YNAM)?;
        let sound_drop: Option<TypedFormId<SOUN>> = parser.try_parse(ZNAM)?;
        Ok(Self {
            editor_id,
            object_bounds,
            name,
            model_data,
            large_icon_file_name,
            small_icon_file_name,
            destruction_data,
            sound_pick_up,
            sound_drop,
        })
    }
}
