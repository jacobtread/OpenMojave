use super::{prelude::*, scpt::SCPT, soun::SOUN};
use crate::esp::record::sub::{
    destruction::DestructionData, model::ModelData, object_bounds::ObjectBounds,
};

/// Misc Item
#[derive(Debug)]
pub struct MISC {
    pub editor_id: EditorId,
    pub object_bounds: ObjectBounds,
    pub name: Option<String>,
    pub model_data: Option<ModelData>,
    pub large_icon_file_name: Option<String>,
    pub small_icon_file_name: Option<String>,
    pub script: Option<TypedFormId<SCPT>>,
    pub destruction_data: Option<DestructionData>,
    pub sound_pick_up: Option<TypedFormId<SOUN>>,
    pub sound_drop: Option<TypedFormId<SOUN>>,
    pub data: MiscData,
    pub sound_random: Option<TypedFormId<SOUN>>,
}

impl Record for MISC {
    const TYPE: RecordType = RecordType::new(b"MISC");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let object_bounds: ObjectBounds = parser.parse(OBND)?;
        let name: Option<String> = parser.try_parse(FULL)?;
        let model_data: Option<ModelData> = ModelData::parse_first(parser)?;
        let large_icon_file_name: Option<String> = parser.try_parse(ICON)?;
        let small_icon_file_name: Option<String> = parser.try_parse(MICO)?;
        let script: Option<TypedFormId<SCPT>> = parser.try_parse(SCRI)?;
        let destruction_data: Option<DestructionData> = DestructionData::parse_next(parser)?;
        let sound_pick_up: Option<TypedFormId<SOUN>> = parser.try_parse(YNAM)?;
        let sound_drop: Option<TypedFormId<SOUN>> = parser.try_parse(ZNAM)?;
        let data: MiscData = parser.parse(DATA)?;
        let sound_random: Option<TypedFormId<SOUN>> = parser.try_parse(RNAM)?;

        Ok(Self {
            editor_id,
            object_bounds,
            name,
            model_data,
            large_icon_file_name,
            small_icon_file_name,
            script,
            destruction_data,
            sound_pick_up,
            sound_drop,
            data,
            sound_random,
        })
    }
}

#[derive(Debug)]
pub struct MiscData {
    pub value: i32,
    pub weight: f32,
}

impl FromRecordBytes for MiscData {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(tuple((le_i32, le_f32)), |(value, weight)| Self {
            value,
            weight,
        })(input)
    }
}
