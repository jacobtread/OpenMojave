use bitflags::bitflags;
use nom::{
    bytes::complete::take,
    combinator::map,
    number::complete::{le_i32, u8},
    sequence::tuple,
};

use crate::esp::{
    record::{
        sub::{
            effect::Effect, equipment_type::EquipmentType, model::ModelData,
            object_bounds::ObjectBounds, DATA, EDID, ENIT, ETYP, FULL, ICON, MICO, OBND, SCRI,
        },
        FromRecordBytes, Record, RecordParseError, RecordParser, RecordType,
    },
    shared::{EditorId, TypedFormId},
};

use super::scpt::SCPT;

/// Ingredient
#[derive(Debug)]
pub struct INGR {
    pub editor_id: EditorId,
    pub object_bounds: ObjectBounds,
    pub name: Option<String>,
    pub model_data: Option<ModelData>,
    pub large_icon_file_name: Option<String>,
    pub small_icon_file_name: Option<String>,
    pub script: Option<TypedFormId<SCPT>>,
    pub equipment_type: EquipmentType,
    pub weight: f32,
    pub effect_data: ENIT,
    pub effects: Vec<Effect>,
}

impl Record for INGR {
    const TYPE: RecordType = RecordType::new(b"INGR");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let object_bounds: ObjectBounds = parser.parse(OBND)?;
        let name: Option<String> = parser.try_parse(FULL)?;
        let model_data: Option<ModelData> = ModelData::parse_first(parser)?;
        let large_icon_file_name: Option<String> = parser.try_parse(ICON)?;
        let small_icon_file_name: Option<String> = parser.try_parse(MICO)?;
        let script: Option<TypedFormId<SCPT>> = parser.try_parse(SCRI)?;
        let equipment_type: EquipmentType = parser.parse(ETYP)?;
        let weight: f32 = parser.parse(DATA)?;
        let effect_data: ENIT = parser.parse(ENIT)?;
        let effects: Vec<Effect> = parser.parse_collection()?;
        if effects.is_empty() {
            return Err(crate::esp::record::RecordParseError::Custom(
                "Missing INGR effects".to_string(),
            ));
        }

        Ok(Self {
            editor_id,
            object_bounds,
            name,
            model_data,
            large_icon_file_name,
            small_icon_file_name,
            script,
            equipment_type,
            weight,
            effect_data,
            effects,
        })
    }
}

#[derive(Debug)]
pub struct ENIT {
    pub value: i32,
    pub flags: ENITFlags,
}

impl FromRecordBytes for ENIT {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(
            tuple((le_i32, ENITFlags::parse, take(3usize))),
            |(value, flags, _)| Self { value, flags },
        )(input)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct ENITFlags: u8 {
        const NO_AUTO_CALCULATION   = 0x01;
        const FOOD_ITEM = 0x02;
    }
}

impl FromRecordBytes for ENITFlags {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}
