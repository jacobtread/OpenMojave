use num_enum::TryFromPrimitive;

use crate::esp::{
    record::{
        enum_value,
        sub::{model::ModelData, object_bounds::ObjectBounds, BRUS, EDID, OBND, RNAM},
        FromRecordBytes, Record, RecordParseError, RecordParser, RecordType,
    },
    shared::{EditorId, TypedFormId},
};

use super::soun::SOUN;

#[derive(Debug)]
pub struct STAT {
    pub editor_id: EditorId,
    pub object_bounds: ObjectBounds,
    pub model_data: Option<ModelData>,
    pub passthrough_sound: Option<BRUS>,
    pub sound_random: Option<TypedFormId<SOUN>>,
}

impl Record for STAT {
    const TYPE: RecordType = RecordType::from_value(b"STAT");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let object_bounds: ObjectBounds = parser.parse(OBND)?;
        let model_data: Option<ModelData> = ModelData::parse_first(parser)?;
        let passthrough_sound: Option<BRUS> = parser.try_parse(BRUS)?;
        let sound_random: Option<TypedFormId<SOUN>> = parser.try_parse(RNAM)?;

        Ok(Self {
            editor_id,
            object_bounds,
            model_data,
            passthrough_sound,
            sound_random,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(i8)]
pub enum BRUS {
    None = -1,
    BushA = 0,
    BushB = 1,
    BushC = 2,
    BushD = 3,
    BushE = 4,
    BushF = 5,
    BushG = 6,
    BushH = 7,
    BushI = 8,
    BushJ = 9,
}

impl FromRecordBytes for BRUS {
    #[inline]
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        enum_value(input)
    }
}
