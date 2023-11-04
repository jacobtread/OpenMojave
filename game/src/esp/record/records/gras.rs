use bitflags::bitflags;
use nom::bytes::complete::take;
use num_enum::TryFromPrimitive;

use crate::esp::{
    record::records::prelude::*,
    record::{
        enum_value,
        sub::{model::ModelData, object_bounds::ObjectBounds, DATA, EDID, OBND},
    },
    shared::EditorId,
};

/// Grass
#[derive(Debug)]
pub struct GRAS {
    pub editor_id: EditorId,
    pub object_bounds: ObjectBounds,
    pub model_data: ModelData,
    pub data: GRASDATA,
}

impl Record for GRAS {
    const TYPE: RecordType = RecordType::from_value(b"GRAS");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let object_bounds: ObjectBounds = parser.parse(OBND)?;
        let model_data: ModelData = ModelData::parse_first(parser)?
            .ok_or_else(|| RecordParseError::Custom("GRAS missing model data".to_string()))?;
        let data: GRASDATA = parser.parse(DATA)?;

        Ok(Self {
            editor_id,
            object_bounds,
            model_data,
            data,
        })
    }
}

#[derive(Debug)]
pub struct GRASDATA {
    pub density: u8,
    pub min_slope: u8,
    pub max_slope: u8,
    pub unit_from_water_amount: u16,
    pub unit_from_water_ty: UnitFromWaterType,
    pub position_range: f32,
    pub height_range: f32,
    pub color_range: f32,
    pub wave_period: f32,
    pub flags: GRASFlags,
}

impl FromRecordBytes for GRASDATA {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((
                u8,
                u8,
                u8,
                u8,
                le_u16,
                le_u16,
                enum_value::<UnitFromWaterType>,
                le_f32,
                le_f32,
                le_f32,
                le_f32,
                GRASFlags::parse,
                take(3usize),
            )),
            |(
                density,
                min_slope,
                max_slope,
                _,
                unit_from_water_amount,
                _,
                unit_from_water_ty,
                position_range,
                height_range,
                color_range,
                wave_period,
                flags,
                _,
            )| Self {
                density,
                min_slope,
                max_slope,
                unit_from_water_amount,
                unit_from_water_ty,
                position_range,
                height_range,
                color_range,
                wave_period,
                flags,
            },
        )(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum UnitFromWaterType {
    AboveAtLeast = 0,
    AboveAtMost = 1,
    BelowAtLeast = 2,
    BelowAtMost = 3,
    EitherAtLeast = 4,
    EitherAtMost = 5,
    EitherAtMostAbove = 6,
    EitherAtMostBelow = 7,
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct GRASFlags: u8 {
        const VERTEX_LIGHTING   = 0x01;
        const UNIFORM_SCALING  = 0x02;
        const FIT_TOSLOPE  = 0x04;
    }
}

impl FromRecordBytes for GRASFlags {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}
