use super::{
    prelude::{model::ModelData, object_bounds::ObjectBounds, *},
    scpt::SCPT,
    soun::SOUN,
};

/// Constructible Object
#[derive(Debug)]
pub struct COBJ {
    pub editor_id: Option<EditorId>,
    pub object_bounds: Option<ObjectBounds>,
    pub name: Option<String>,
    pub model_data: Option<ModelData>,
    pub large_icon_file_name: Option<String>,
    pub small_icon_file_name: Option<String>,
    pub script: Option<TypedFormId<SCPT>>,
    pub sound_pick_up: Option<TypedFormId<SOUN>>,
    pub sound_drop: Option<TypedFormId<SOUN>>,
    pub data: COBJData,
}

impl Record for COBJ {
    const TYPE: RecordType = COBJ;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: Option<EditorId> = parser.try_parse(EDID)?;
        let object_bounds: Option<ObjectBounds> = parser.try_parse(OBND)?;
        let name: Option<String> = parser.try_parse(FULL)?;
        let model_data: Option<ModelData> = ModelData::parse_first(parser)?;
        let large_icon_file_name: Option<String> = parser.try_parse(ICON)?;
        let small_icon_file_name: Option<String> = parser.try_parse(MICO)?;
        let script: Option<TypedFormId<SCPT>> = parser.try_parse(SCRI)?;
        let sound_pick_up: Option<TypedFormId<SOUN>> = parser.try_parse(YNAM)?;
        let sound_drop: Option<TypedFormId<SOUN>> = parser.try_parse(ZNAM)?;
        let data: COBJData = parser.parse(DATA)?;

        Ok(Self {
            editor_id,
            object_bounds,
            name,
            model_data,
            large_icon_file_name,
            small_icon_file_name,
            script,
            sound_pick_up,
            sound_drop,
            data,
        })
    }
}

#[derive(Debug)]
pub struct COBJData {
    pub value: i32,
    pub weight: f32,
}

impl FromRecordBytes for COBJData {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(tuple((le_i32, le_f32)), |(value, weight)| Self {
            value,
            weight,
        })(input)
    }
}
