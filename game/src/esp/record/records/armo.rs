use bitflags::bitflags;
use nom::{
    bytes::complete::take,
    combinator::{map, rest},
    number::complete::{le_f32, le_i16, le_i32, le_u16, le_u32, u8},
    sequence::tuple,
    IResult,
};
use num_enum::TryFromPrimitive;

use crate::esp::{
    record::{
        enum_value,
        sub::{
            equipment_type::EquipmentType, model::ModelData, object_bounds::ObjectBounds, BIPL,
            BMCT, BMDT, BNAM, DATA, DNAM, EDID, EITM, ETYP, FULL, ICO2, ICON, MIC2, MICO, OBND,
            REPL, SCRI, SNAM, TNAM, YNAM, ZNAM,
        },
        take4, FromRecordBytes, Record, RecordParseError, RecordParser, RecordType,
    },
    shared::{EditorId, FormId, TypedFormId},
};

use super::{scpt::SCPT, soun::SOUN};

/// Armor
#[derive(Debug)]
pub struct ARMO {
    pub editor_id: EditorId,
    pub object_bounds: ObjectBounds,
    pub name: Option<String>,
    pub script: Option<TypedFormId<SCPT>>,
    pub object_effect: Option<FormId /* ENCH / SPEL */>,
    pub biped_data: BMDT,
    pub male_biped_model_data: ModelData,
    pub male_world_model_data: ModelData,
    pub male_inventory_icon_file_name: Option<String>,
    pub male_message_icon_file_name: Option<String>,
    pub female_biped_model_data: ModelData,
    pub female_world_model_data: ModelData,
    pub female_inventory_icon_file_name: Option<String>,
    pub female_message_icon_file_name: Option<String>,
    pub ragdoll_constraint_template: Option<String>,
    pub repair_list: Option<TypedFormId<() /* FLST */>>,
    pub biped_model_list: Option<TypedFormId<() /* FLST */>>,
    pub equipment_type: EquipmentType,
    pub sound_pickup: Option<TypedFormId<SOUN>>,
    pub sound_drop: Option<TypedFormId<SOUN>>,
    pub data: ArmorData,
    pub dnam: DNAM,
    pub overrides_animation_sound: bool,
    pub animation_sound: Vec<SNAM>,
    pub animation_sounds_template: Option<TypedFormId<ARMO>>,
}

impl Record for ARMO {
    const TYPE: RecordType = RecordType::from_value(b"ARMO");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let object_bounds: ObjectBounds = parser.parse(OBND)?;
        let name: Option<String> = parser.try_parse(FULL)?;
        let script: Option<TypedFormId<SCPT>> = parser.try_parse(SCRI)?;
        let object_effect: Option<FormId> = parser.try_parse(EITM)?;
        let biped_data: BMDT = parser.parse(BMDT)?;

        let male_biped_model_data: ModelData = ModelData::parse_first(parser)?
            .ok_or_else(|| RecordParseError::Custom("Missing male_biped_model_data".to_string()))?;
        let male_world_model_data: ModelData = ModelData::parse_second(parser)?
            .ok_or_else(|| RecordParseError::Custom("Missing male_world_model_data".to_string()))?;

        let male_inventory_icon_file_name: Option<String> = parser.try_parse(ICON)?;
        let male_message_icon_file_name: Option<String> = parser.try_parse(MICO)?;

        let female_biped_model_data: ModelData = ModelData::parse_third(parser)?
            .ok_or_else(|| RecordParseError::Custom("Missing male_biped_model_data".to_string()))?;
        let female_world_model_data: ModelData = ModelData::parse_fourth(parser)?
            .ok_or_else(|| RecordParseError::Custom("Missing male_world_model_data".to_string()))?;

        let female_inventory_icon_file_name: Option<String> = parser.try_parse(ICO2)?;
        let female_message_icon_file_name: Option<String> = parser.try_parse(MIC2)?;

        let ragdoll_constraint_template: Option<String> = parser.try_parse(BMCT)?;
        let repair_list: Option<TypedFormId<()>> = parser.try_parse(REPL)?;
        let biped_model_list: Option<TypedFormId<()>> = parser.try_parse(BIPL)?;
        let equipment_type: EquipmentType = parser.parse(ETYP)?;
        let sound_pickup: Option<TypedFormId<SOUN>> = parser.try_parse(YNAM)?;
        let sound_drop: Option<TypedFormId<SOUN>> = parser.try_parse(ZNAM)?;
        let data: ArmorData = parser.parse(DATA)?;
        let dnam: DNAM = parser.parse(DNAM)?;
        let overrides_animation_sound: bool = parser.try_parse(BNAM)?.unwrap_or_default();
        let animation_sound: Vec<SNAM> = parser.try_parse_many(SNAM)?;
        let animation_sounds_template: Option<TypedFormId<ARMO>> = parser.try_parse(TNAM)?;

        Ok(Self {
            editor_id,
            object_bounds,
            name,
            script,
            object_effect,
            biped_data,
            male_biped_model_data,
            male_world_model_data,
            male_inventory_icon_file_name,
            male_message_icon_file_name,
            female_biped_model_data,
            female_world_model_data,
            female_inventory_icon_file_name,
            female_message_icon_file_name,
            ragdoll_constraint_template,
            repair_list,
            biped_model_list,
            equipment_type,
            sound_pickup,
            sound_drop,
            data,
            dnam,
            overrides_animation_sound,
            animation_sound,
            animation_sounds_template,
        })
    }
}

#[derive(Debug)]
pub struct ArmorData {
    pub value: i32,
    pub max_condition: i32,
    pub weight: f32,
}

impl FromRecordBytes for ArmorData {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((le_i32, le_i32, le_f32)),
            |(value, max_condition, weight)| Self {
                value,
                max_condition,
                weight,
            },
        )(input)
    }
}

#[derive(Debug)]
pub struct DNAM {
    pub ar: i16,
    pub flags: DNAMFlags,
    pub dt: f32,
    pub unknown: [u8; 4],
}

impl FromRecordBytes for DNAM {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((le_i16, DNAMFlags::parse, le_f32, take4)),
            |(ar, flags, dt, unknown)| Self {
                ar,
                flags,
                dt,
                unknown,
            },
        )(input)
    }
}

bitflags! {
    #[derive(Debug, Clone)]
    pub struct DNAMFlags: u16 {
        const MODULATES_VOICE = 0x0001;
    }
}

impl FromRecordBytes for DNAMFlags {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(le_u16, Self::from_bits_retain)(input)
    }
}

#[derive(Debug)]
pub struct SNAM {
    pub sound: TypedFormId<SOUN>,
    pub chance: u8,
    pub ty: SNAMType,
}

impl FromRecordBytes for SNAM {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((TypedFormId::parse, u8, take(3usize), enum_value::<SNAMType>)),
            |(sound, chance, _, ty)| Self { sound, chance, ty },
        )(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum SNAMType {
    Run = 19,
    RunInArmor = 20,
    Sneak = 21,
    SneakInArmor = 22,
    Walk = 23,
    WalkInArmor = 24,
}

#[derive(Debug)]
pub struct BMDT {
    pub biped_flags: BipedFlags,
    pub general_flags: GeneralFlags,
}

impl FromRecordBytes for BMDT {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((BipedFlags::parse, GeneralFlags::parse, rest)),
            |(biped_flags, general_flags, _)| Self {
                biped_flags,
                general_flags,
            },
        )(input)
    }
}

bitflags! {
    #[derive(Debug, Clone)]
    pub struct BipedFlags: u32 {
        const HEAD = 0x00000001;
        const HAIR = 0x00000002;
        const UPPER_BODY = 0x00000004;
        const LEFT_HAND = 0x00000008;
        const RIGHT_HAND = 0x00000010;
        const WEAPON = 0x00000020;
        const PIP_BOY = 0x00000040;
        const BACKPACK = 0x00000080;
        const NECKLACE = 0x00000100;
        const HEADBAND = 0x00000200;
        const HAT = 0x00000400;
        const EYE_GLASSES = 0x00000800;
        const NOSE_RING = 0x00001000;
        const EARRINGS = 0x00002000;
        const MASK = 0x00004000;
        const CHOKER = 0x00008000;
        const MOUTH_OBJECT = 0x00010000;
        const BODY_ADD_ON_1 = 0x00020000;
        const BODY_ADD_ON_2 = 0x00040000;
        const BODY_ADD_ON_3 = 0x00080000;
    }
}

impl FromRecordBytes for BipedFlags {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(le_u32, Self::from_bits_retain)(input)
    }
}
bitflags! {
    #[derive(Debug, Clone)]
    pub struct GeneralFlags: u8 {
        const U1 = 0x01;
        const U2 = 0x02;
        const HAS_BACKPACK = 0x04;
        const MEDIUM = 0x08;
        const U3 = 0x10;
        const POWER_ARMOR = 0x20;
        const NON_PLAYABLE = 0x40;
        const HEAVY = 0x80;
    }
}

impl FromRecordBytes for GeneralFlags {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}
