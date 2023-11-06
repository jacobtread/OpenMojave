use bitflags::bitflags;
use fyrox::core::algebra::Vector2;
use num_enum::TryFromPrimitive;

use crate::esp::{
    record::{
        sub::{
            destruction::DestructionData, equipment_type::EquipmentType, model::ModelData,
            object_bounds::ObjectBounds, sound_level::SoundLevel, BNAM, CNAM, EDID, FULL, ICON,
            MICO, MNAM, OBND, SCRI, SNAM,
        },
        Repeated,
    },
    shared::{EditorId, FormId, TypedFormId},
};

pub use super::prelude::*;
use super::{scpt::SCPT, soun::SOUN, stat::STAT};

#[derive(Debug)]
pub struct WEAP {
    pub editor_id: EditorId,
    pub object_bounds: ObjectBounds,
    pub name: Option<String>,
    pub model_data: ModelData,
    pub large_icon_file_name: Option<String>,
    pub small_icon_file_name: Option<String>,
    pub script: Option<TypedFormId<SCPT>>,
    /// FormID of an ENCH or SPEL record.
    pub effect: Option<FormId>,
    pub enchantment_charge_amount: i16,
    /// FormID of an AMMO or FLST record.
    pub ammo: FormId,
    pub destruction_data: Option<DestructionData>,
    pub repair_list: Option<TypedFormId<() /* FLST */>>,
    pub equipment_type: EquipmentType,
    pub biped_model_list: Option<TypedFormId<() /* FLST */>>,
    pub sound_pick_up: Option<TypedFormId<SOUN>>,
    pub sound_drop: Option<TypedFormId<SOUN>>,
    pub shell_casing_model_data: Option<ModelData>,
    pub scope_model_data: Option<ModelData>,
    pub scope_effect: Option<TypedFormId<() /* EFSH */>>,
    pub scope_effect_model_data: Option<ModelData>,
    pub model_with_mod_1: Option<String>,
    pub model_with_mod_2: Option<String>,
    pub model_with_mod_1_2: Option<String>,
    pub model_with_mod_3: Option<String>,
    pub model_with_mod_1_3: Option<String>,
    pub model_with_mod_2_3: Option<String>,
    pub model_with_mod_1_2_3: Option<String>,
    pub vats_attack_name: Option<String>,
    pub embedded_weapon_node: Option<String>,
    pub impact_data_set: Option<TypedFormId<() /* IPDS */>>,

    pub first_person_model: Option<TypedFormId<STAT>>,
    pub first_person_model_with_mod_1: Option<TypedFormId<STAT>>,
    pub first_person_model_with_mod_2: Option<TypedFormId<STAT>>,
    pub first_person_model_with_mod_1_2: Option<TypedFormId<STAT>>,
    pub first_person_model_with_mod_3: Option<TypedFormId<STAT>>,
    pub first_person_model_with_mod_1_3: Option<TypedFormId<STAT>>,
    pub first_person_model_with_mod_2_3: Option<TypedFormId<STAT>>,
    pub first_person_model_with_mod_1_2_3: Option<TypedFormId<STAT>>,

    pub weapon_mod_1: Option<TypedFormId<() /* IMOD */>>,
    pub weapon_mod_2: Option<TypedFormId<() /* IMOD */>>,
    pub weapon_mod_3: Option<TypedFormId<() /* IMOD */>>,

    pub sound_gun_shoot_3d: Option<TypedFormId<SOUN>>,
    pub sound_gun_shoot_dist: Option<TypedFormId<SOUN>>,
    pub sound_gun_shoot_2d: Option<TypedFormId<SOUN>>,
    pub sound_gun_shoot_3d_looping: Option<TypedFormId<SOUN>>,
    pub sound_melee_no_ammo: Option<TypedFormId<SOUN>>,
    pub sound_block: Option<TypedFormId<SOUN>>,
    pub sound_idle: Option<TypedFormId<SOUN>>,
    pub sound_equip: Option<TypedFormId<SOUN>>,
    pub sound_unequip: Option<TypedFormId<SOUN>>,
    pub sound_mod_1_shoot_3d: Option<TypedFormId<SOUN>>,
    pub sound_mod_1_shoot_dist: Option<TypedFormId<SOUN>>,
    pub sound_mod_1_shoot_2d: Option<TypedFormId<SOUN>>,
    pub data: (),
    pub dnam: DNAM,
    pub critial_data: (),
    pub vats: Option<()>,
    pub sound_level: SoundLevel,
}

#[derive(Debug)]
pub struct DATA {
    pub value: i32,
    pub health: i32,
    pub weight: f32,
    pub base_damage: i16,
    pub clip_size: u8,
}

#[derive(Debug)]
pub struct DNAM {
    pub animation_type: WeaponAnimationType,
    pub animation_multiplier: f32,
    pub reach: f32,
    pub flags_1: Flags1,
    pub grip_animation: GripAnimation,
    pub ammo_use: u8,
    pub reload_animation: u8,
    pub min_spread: f32,
    pub spread: f32,
    pub sight_fov: f32,
    pub projectile: TypedFormId<() /* PROJ */>,
    pub base_vats_to_hit_chance: u8,
    pub attack_animation: u8,
    pub projectile_cloud: u8,
    pub embedded_weapon_actor_value: u8,
    /// Min, Max
    pub range: Vector2<f32>,
    pub on_hit: u32,
    pub flags_2: u32,
    pub animation_attack_multipler: f32,
    pub fire_rate: f32,
    pub override_action_points: f32,
    /// Left, Right
    pub rumble_motor_strength: Vector2<f32>,
    pub rumble_duration: f32,
    pub override_damage_to_weapon_mult: f32,
    pub attack_shots_sec: f32,
    pub reload_time: f32,
    pub jam_time: f32,
    pub aim_arc: f32,
    // TODO: Enum
    pub skill: i32,
    pub rumble_pattern: u32,
    pub rumble_wave_length: f32,
    pub limb_damage_multipler: f32,
    // TODO: Enum
    pub resistance_type: i32,
    pub sight_usage: f32,
    // Min, Max
    pub semi_auto_fire_delay: Vector2<f32>,
    // TODO: Enum
    pub effect_mod_1: u32,
    // TODO: Enum
    pub effect_mod_2: u32,
    // TODO: Enum
    pub effect_mod_3: u32,
    pub power_attack_animation_override: u32,
    pub strength_requirement: u32,
    // TODO: Enum
    pub reload_animation_mod: u32,
    pub regen_rate: f32,
    pub kill_impulse: f32,
    pub value_b_mod_1: f32,
    pub value_b_mod_2: f32,
    pub value_b_mod_3: f32,
    pub impulse_dist: f32,
    pub skill_requirement: u32,
}

#[derive(Debug, Clone, Copy, TryFromPrimitive, PartialEq, Eq)]
#[repr(u32)]
pub enum WeaponAnimationType {
    HandToHand = 0,
    Melee1Hand = 1,
    Melee2Hand = 2,
    PistolBallistic = 3,
    PistolEnergy = 4,
    RilfeBallistic = 5,
    RilfeAutomatic = 6,
    RilfeEnergy = 7,
    Handle = 8,
    Launcher = 9,
    GrenadeThrow = 10,
    LandMine = 11,
    MineDrop = 12,
    Thrown = 13,
}

#[derive(Debug, Clone, Copy, TryFromPrimitive, PartialEq, Eq)]
#[repr(u8)]
pub enum GripAnimation {
    HandGrip1 = 230,
    HandGrip2 = 231,
    HandGrip3 = 232,
    HandGrip4 = 233,
    HandGrip5 = 234,
    HandGrip6 = 235,
    Default = 255,
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct Flags1: u8 {
        const IGNORES_NORMAL_WEAPON_RESISTANCE   = 0x01;
        const IS_AUTOMATIC  = 0x02;
        const HAS_SCOPE  = 0x04;
        const CANT_DROP = 0x08;
        const HIDE_BACKPACK = 0x10;
        const EMBEDDED_WEAPON= 0x20;
        const DONT_USE_1ST_PERSON_IS_ANIMS = 0x40;
        const NON_PLAYABLE= 0x80;
    }
}

impl FromRecordBytes for Flags1 {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}
