use super::{
    prelude::{destruction::DestructionData, model::ModelData, object_bounds::ObjectBounds, *},
    scpt::SCPT,
    soun::SOUN,
};

/// Key
#[derive(Debug)]
pub struct KEYM {
    pub editor_id: EditorId,
    pub object_bounds: ObjectBounds,
    pub name: String,
    pub model_data: Option<ModelData>,
    pub large_icon_file_name: String,
    pub small_icon_file_name: String,
    pub script: Option<TypedFormId<SCPT>>,
    pub destruction_data: Option<DestructionData>,
    pub sound_pick_up: Option<TypedFormId<SOUN>>,
    pub sound_drop: Option<TypedFormId<SOUN>>,
    pub data: Option<KeyData>,
    pub sound_random: Option<TypedFormId<SOUN>>,
}

impl Record for KEYM {
    const TYPE: RecordType = RecordType::new(b"KEYM");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let object_bounds: ObjectBounds = parser.parse(OBND)?;
        let name: String = parser.parse(FULL)?;
        let model_data: Option<ModelData> = ModelData::parse_first(parser)?;
        let large_icon_file_name: String = parser.parse(ICON)?;
        let small_icon_file_name: String = parser.parse(MICO)?;
        let script: Option<TypedFormId<SCPT>> = parser.try_parse(SCRI)?;
        let destruction_data: Option<DestructionData> = DestructionData::parse_next(parser)?;
        let sound_pick_up: Option<TypedFormId<SOUN>> = parser.try_parse(YNAM)?;
        let sound_drop: Option<TypedFormId<SOUN>> = parser.try_parse(ZNAM)?;
        let data: Option<KeyData> = parser.try_parse(DATA)?;
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
pub struct KeyData {
    pub value: i32,
    pub weight: f32,
}

impl FromRecordBytes for KeyData {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(tuple((le_i32, le_f32)), |(value, weight)| Self {
            value,
            weight,
        })(input)
    }
}
