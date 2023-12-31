use super::{prelude::*, scpt::SCPT, soun::SOUN};
use crate::esp::record::sub::{model::ModelData, object_bounds::ObjectBounds};

/// Light
#[derive(Debug)]
pub struct LIGH {
    pub editor_id: EditorId,
    pub object_bounds: ObjectBounds,
    pub model_data: Option<ModelData>,
    pub script: Option<TypedFormId<SCPT>>,
    pub name: Option<String>,
    pub large_icon_file_name: Option<String>,
    pub small_icon_file_name: Option<String>,
    pub data: LightData,
    pub fade_value: f32,
    pub sound: Option<TypedFormId<SOUN>>,
}

impl Record for LIGH {
    const TYPE: RecordType = RecordType::new(b"LIGH");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let object_bounds: ObjectBounds = parser.parse(OBND)?;
        let model_data: Option<ModelData> = ModelData::parse_first(parser)?;
        let script: Option<TypedFormId<SCPT>> = parser.try_parse(SCRI)?;
        let name: Option<String> = parser.try_parse(FULL)?;
        let large_icon_file_name: Option<String> = parser.try_parse(ICON)?;
        let small_icon_file_name: Option<String> = parser.try_parse(MICO)?;
        let data: LightData = parser.parse(DATA)?;
        let fade_value: f32 = parser.parse(FNAM)?;
        let sound: Option<TypedFormId<SOUN>> = parser.try_parse(SNAM)?;

        Ok(Self {
            editor_id,
            object_bounds,
            model_data,
            script,
            name,
            large_icon_file_name,
            small_icon_file_name,
            data,
            fade_value,
            sound,
        })
    }
}

#[derive(Debug)]
pub struct LightData {
    pub time: i32,
    pub radius: u32,
    pub color: RGBA,
    pub flags: LightFlags,
    pub falloff_exponent: f32,
    pub fov: f32,
    pub value: u32,
    pub weight: f32,
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct LightFlags: u32 {
        const DYNAMIC   = 0x00000001;
        const CAN_BE_CARRIED = 0x00000002;
        const NEGATIVE = 0x00000004;
        const FLICKER = 0x00000008;
        const UNUSED = 0x00000010;
        const OFF_BY_DEFAULT = 0x00000020;
        const FLICKER_SLOW = 0x00000040;
        const PULSE = 0x00000080;
        const PULSE_SLOW = 0x00000100;
        const SPOT_LIGHT = 0x00000200;
        const SPOT_SHADOW = 0x00000400;
    }
}

impl FromRecordBytes for LightData {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(
            tuple((
                le_i32,
                le_u32,
                RGBA::parse,
                LightFlags::parse,
                le_f32,
                le_f32,
                le_u32,
                le_f32,
            )),
            |(time, radius, color, flags, falloff_exponent, fov, value, weight)| Self {
                time,
                radius,
                color,
                flags,
                falloff_exponent,
                fov,
                value,
                weight,
            },
        )(input)
    }
}

impl FromRecordBytes for LightFlags {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(le_u32, Self::from_bits_retain)(input)
    }
}
