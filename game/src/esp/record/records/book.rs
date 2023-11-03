use bitflags::bitflags;
use nom::{
    combinator::map,
    number::complete::{le_f32, le_i32, u8},
    sequence::tuple,
};

use crate::esp::{
    record::{
        enum_value,
        sub::{
            destruction::DestructionData, model::ModelData, object_bounds::ObjectBounds,
            skill::Skill, DATA, DESC, EDID, FULL, ICON, MICO, OBND, SCRI, YNAM, ZNAM,
        },
        FromRecordBytes, Record, RecordCollection, RecordParseError, RecordParser, RecordType,
    },
    shared::{EditorId, TypedFormId},
};

use super::{scpt::SCPT, soun::SOUN};

/// Book
#[derive(Debug)]
pub struct BOOK {
    pub editor_id: EditorId,
    pub object_bounds: ObjectBounds,
    pub name: Option<String>,
    pub model_data: ModelData,
    pub large_icon_file_name: Option<String>,
    pub small_icon_file_name: Option<String>,
    pub script: Option<TypedFormId<SCPT>>,
    pub description: String,
    pub destruction_data: Option<DestructionData>,
    pub sound_pick_up: Option<TypedFormId<SOUN>>,
    pub sound_drop: Option<TypedFormId<SOUN>>,
    pub data: BOOKData,
}

impl Record for BOOK {
    const TYPE: RecordType = RecordType::from_value(b"BOOK");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let object_bounds: ObjectBounds = parser.parse(OBND)?;
        let name: Option<String> = parser.try_parse(FULL)?;
        let model_data: ModelData = ModelData::parse_first(parser)?
            .ok_or_else(|| RecordParseError::Custom("BOOK missing model data".to_string()))?;
        let large_icon_file_name: Option<String> = parser.try_parse(ICON)?;
        let small_icon_file_name: Option<String> = parser.try_parse(MICO)?;
        let script: Option<TypedFormId<SCPT>> = parser.try_parse(SCRI)?;
        let description: String = parser.parse(DESC)?;
        let destruction_data: Option<DestructionData> = DestructionData::parse_next(parser)?;
        let sound_pick_up: Option<TypedFormId<SOUN>> = parser.try_parse(YNAM)?;
        let sound_drop: Option<TypedFormId<SOUN>> = parser.try_parse(ZNAM)?;
        let data: BOOKData = parser.parse(DATA)?;

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
pub struct BOOKData {
    pub flags: BOOKFlags,
    pub skill: Skill,
    pub value: i32,
    pub weight: f32,
}

impl FromRecordBytes for BOOKData {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(
            tuple((BOOKFlags::parse, enum_value::<Skill>, le_i32, le_f32)),
            |(flags, skill, value, weight)| Self {
                flags,
                skill,
                value,
                weight,
            },
        )(input)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct BOOKFlags: u8 {
        const UNKNOWN   = 0x01;
        const NOT_TAKABLE  = 0x02;
    }
}

impl FromRecordBytes for BOOKFlags {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}
