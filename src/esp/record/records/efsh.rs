use super::prelude::*;

/// Effect Shader
#[derive(Debug)]
pub struct EFSH {
    // TODO:
}

impl Record for EFSH {
    const TYPE: RecordType = RecordType::new(b"EFSH");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}

pub struct EffectShaderData {
    pub flags: EffectShaderFlags,
    /// Membrain shader
    pub mb_source_blend_mode: BlendMode,
    pub mb_blend_operation: BlendOperation,
    pub mb_z_test_function: ZTestFunction,
    // Fill / Texture Effect
    pub te_color: RGBA,
    pub te_alpha_fade: AlphaFade,
    pub te_texture_animation_speed_u: f32,
    pub te_texture_animation_speed_v: f32,
    // Edge Effect
    pub ee_fall_off: f32,
    pub ee_color: RGBA,
    pub ee_alpha_fade: AlphaFade,
    pub ee_te_full_alpha_ratio: f32,
    pub ee_full_alpha_ratio: f32,

    // Membrain shader
    pub mb_dest_blend_mode: BlendMode,

    // Particle Shader
    pub pt_source_blend_mode: BlendMode,
    pub pt_blend_operation: BlendMode,
    pub pt_z_test_function: ZTestFunction,
    pub pt_dest_blend_mode: BlendMode,
    pub pt_particle_birth_ramp_up_time: f32,
    pub pt_full_particle_birth_up_time: f32,
    // TODO:
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct EffectShaderFlags: u8 {
        const NO_MEMBRANE_SHADER              = 0x01;
        const U1                              = 0x02;
        const U2                              = 0x04;
        const NO_PARTICLE_SHADER              = 0x08;
        const EDGE_EFFECT_INVERSE             = 0x10;
        const MEMBRANE_SHADER_AFFECT_SKIN_ONL = 0x20;
    }
}
pub struct AlphaFade {
    pub alpha_fade_in_time: f32,
    pub full_alpha_time: f32,
    pub alpha_fade_out_time: f32,
    pub persistent_alpha_ratio: f32,
    pub alpha_pulse_amplitude: f32,
    pub alpha_pulse_frequency: f32,
}
pub struct EdgeEffect {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum BlendMode {
    Unknown = 0,
    Zero = 1,
    One = 2,
    SourceColor = 3,
    SourceInverseColor = 4,
    SourceAlpha = 5,
    SourceInvertedAlpha = 6,
    DestinationAlpha = 7,
    DestinationInvertedAlpha = 8,
    DestinationColor = 9,
    DestinationInverseColor = 10,
    SourceAlphaSAT = 11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum BlendOperation {
    Unknown = 0,
    Add = 1,
    Subtract = 2,
    ReverseSubtract = 3,
    Minimum = 4,
    Maximum = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum ZTestFunction {
    U1 = 0,
    U2 = 1,
    U3 = 2,
    EqualTo = 3,
    Normal = 4,
    GreaterThan = 5,
    U4 = 6,
    GreaterThanOrEqualTo = 7,
    AlwaysShow = 8,
}

impl FromRecordBytes for EffectShaderFlags {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}
