use bitflags::bitflags;
use nom::{
    combinator::map,
    number::complete::{le_f32, le_i32, le_u16, le_u32},
    IResult,
};
use num_enum::TryFromPrimitive;

use crate::esp::{
    record::{
        enum_value,
        sub::{actor_values::ActorValue, model::ModelData, DATA, DESC, EDID, FULL, ICON, MICO},
        FromRecordBytes, Record, RecordParseError, RecordParser, RecordType,
    },
    shared::{EditorId, FormId, TypedFormId},
};

use super::soun::SOUN;

/// Magic effect
#[derive(Debug)]
pub struct MGEF {
    pub editor_id: EditorId,
    pub name: Option<String>,
    pub description: String,
    pub large_icon_file_name: Option<String>,
    pub small_icon_file_name: Option<String>,
    pub model_data: Option<ModelData>,
    pub data: MagicEffectData,
}

impl Record for MGEF {
    const TYPE: RecordType = RecordType::new(b"MGEF");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id = parser.parse::<EditorId>(EDID)?;
        let name = parser.try_parse::<String>(FULL)?;
        let description = parser.parse::<String>(DESC)?;
        let large_icon_file_name = parser.try_parse::<String>(ICON)?;
        let small_icon_file_name = parser.try_parse::<String>(MICO)?;
        let model_data = ModelData::parse_first(parser)?;
        let data = parser.parse::<MagicEffectData>(DATA)?;

        Ok(Self {
            editor_id,
            name,
            description,
            large_icon_file_name,
            small_icon_file_name,
            model_data,
            data,
        })
    }
}

#[derive(Debug)]
pub struct MagicEffectData {
    pub flags: Flags,
    pub base_cost: f32,
    pub associated_item: FormId,
    pub resistance_type: ActorValue,
    pub light: TypedFormId<() /* LIGH */>,
    pub projectile_speed: f32,
    pub effect_shader: TypedFormId<() /* EFSH */>,
    pub object_display_shader: TypedFormId<() /* EFSH */>,
    pub effect_sound: TypedFormId<SOUN>,
    pub bold_sound: TypedFormId<SOUN>,
    pub hit_sound: TypedFormId<SOUN>,
    pub area_sound: TypedFormId<SOUN>,
    pub archtype: Archtype,
    pub actor_value: ActorValue,
}

impl FromRecordBytes for MagicEffectData {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, flags) = Flags::parse(input)?;
        let (input, base_cost) = le_f32(input)?;
        let (input, associated_item) = FormId::parse(input)?;
        let (input, _magic_school) = le_i32(input)?;
        let (input, resistance_type) = enum_value::<ActorValue>(input)?;
        let (input, _unknown) = le_u16(input)?;
        let (input, _unused) = le_u16(input)?;
        let (input, light) = <TypedFormId<()>>::parse(input)?;
        let (input, projectile_speed) = le_f32(input)?;
        let (input, effect_shader) = <TypedFormId<()>>::parse(input)?;
        let (input, object_display_shader) = <TypedFormId<()>>::parse(input)?;
        let (input, effect_sound) = <TypedFormId<SOUN>>::parse(input)?;
        let (input, bold_sound) = <TypedFormId<SOUN>>::parse(input)?;
        let (input, hit_sound) = <TypedFormId<SOUN>>::parse(input)?;
        let (input, area_sound) = <TypedFormId<SOUN>>::parse(input)?;

        // Constant Effect Enchantment Factor (Unused)
        let (input, _unused) = le_f32(input)?;
        // Constant Effect Barter Factor (Unused)
        let (input, _unused) = le_f32(input)?;

        let (input, archtype) = enum_value::<Archtype>(input)?;
        let (input, actor_value) = enum_value::<ActorValue>(input)?;

        Ok((
            input,
            Self {
                flags,
                base_cost,
                associated_item,
                resistance_type,
                light,
                projectile_speed,
                effect_shader,
                object_display_shader,
                effect_sound,
                bold_sound,
                hit_sound,
                area_sound,
                archtype,
                actor_value,
            },
        ))
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct Flags: u32 {
        const HOSTILE = 0x00000001;
        const RECOVER = 0x00000002;
        const DETRIMENTAL = 0x00000004;
        const U1 = 0x00000008;
        const SELF = 0x00000010;
        const TOUCH = 0x00000020;
        const TARGET = 0x00000040;
        const NO_DURATION = 0x00000080;
        const NO_MAGNITUDE = 0x00000100;
        const NO_AREA = 0x00000200;
        const FX_PERSIST = 0x00000400;
        const U2 = 0x00000800;
        const GORY_VISUALS = 0x00001000;
        const DISPLAY_NAME_ONLY = 0x00002000;
        const U3 = 0x00004000;
        const RADIO_BROADCAST = 0x00008000;
        const U4 = 0x00010000;
        const U5 = 0x00020000;
        const U6 = 0x00040000;
        const USE_SKILL = 0x00080000;
        const USE_ATTRIBUTE = 0x00100000;
        const U7 = 0x00200000;
        const U8 = 0x00400000;
        const U9 = 0x00800000;
        const PAINLESS = 0x01000000;
        /// Spray projectile type (or Fog if Bolt is specified as well)
        const SPRAY_PROJECTILE_TYPE = 0x02000000;
        /// Bolt projectile type (or Fog if Spray is specified as well)
        const BOLT_PROJECTILE_TYPE = 0x04000000;
        const NO_HIT_EFFECT = 0x08000000;
        const NO_DEATH_DISPEL = 0x10000000;
        const U10 = 0x20000000;
    }
}

impl FromRecordBytes for Flags {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(le_u32, Self::from_bits_retain)(input)
    }
}

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(i32)]
pub enum Archtype {
    ValueModifier = 0,
    Script = 1,
    Dispel = 2,
    CureDisease = 3,
    U4 = 4,
    U5 = 5,
    U6 = 6,
    U7 = 7,
    U8 = 8,
    U9 = 9,
    U10 = 10,
    Invisibility = 11,
    Chameleon = 12,
    Light = 13,
    U14 = 14,
    U15 = 15,
    Lock = 16,
    Open = 17,
    BoundItem = 18,
    SummonCreature = 19,
    U20 = 20,
    U21 = 21,
    U22 = 22,
    U23 = 23,
    Paralysis = 24,
    U25 = 25,
    U26 = 26,
    U27 = 27,
    U28 = 28,
    U29 = 29,
    CureParalysis = 30,
    CureAddiction = 31,
    CurePoison = 32,
    Concussion = 33,
    ValueAndParts = 34,
    LimbCondition = 35,
    Turbo = 36,
}
