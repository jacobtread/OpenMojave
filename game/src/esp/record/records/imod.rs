use crate::esp::record::records::prelude::object_bounds::ObjectBounds;

use super::{
    prelude::{destruction::DestructionData, model::ModelData, *},
    scpt::SCPT,
    soun::SOUN,
};

/// Item mod
#[derive(Debug)]
pub struct IMOD {
    pub editor_id: EditorId,
    pub object_bounds: ObjectBounds,
    pub name: Option<String>,
    pub model_data: Option<ModelData>,
    pub large_icon_file_name: Option<String>,
    pub small_icon_file_name: Option<String>,
    pub script: Option<TypedFormId<SCPT>>,
    pub description: Option<String>,
    pub destruction_data: Option<DestructionData>,
    pub sound_pick_up: Option<TypedFormId<SOUN>>,
    pub sound_drop: Option<TypedFormId<SOUN>>,
    pub data: ItemModData,
}

impl Record for IMOD {
    const TYPE: RecordType = RecordType::new(b"IMOD");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let object_bounds: ObjectBounds = parser.parse(OBND)?;
        let name: Option<String> = parser.try_parse(FULL)?;
        let model_data: Option<ModelData> = ModelData::parse_first(parser)?;
        let large_icon_file_name: Option<String> = parser.try_parse(ICON)?;
        let small_icon_file_name: Option<String> = parser.try_parse(MICO)?;
        let script: Option<TypedFormId<SCPT>> = parser.try_parse(SCRI)?;
        let description: Option<String> = parser.try_parse(DESC)?;
        let destruction_data: Option<DestructionData> = DestructionData::parse_next(parser)?;
        let sound_pick_up: Option<TypedFormId<SOUN>> = parser.try_parse(YNAM)?;
        let sound_drop: Option<TypedFormId<SOUN>> = parser.try_parse(ZNAM)?;
        let data: ItemModData = parser.parse(DATA)?;

        Ok(Self {
            editor_id,
            object_bounds,
            name,
            model_data,
            large_icon_file_name,
            small_icon_file_name,
            script,
            description,
            destruction_data,
            sound_pick_up,
            sound_drop,
            data,
        })
    }
}

#[derive(Debug)]
pub struct ItemModData {
    pub value: u32,
    pub weight: f32,
}

impl FromRecordBytes for ItemModData {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(tuple((le_u32, le_f32)), |(value, weight)| Self {
            value,
            weight,
        })(input)
    }
}
