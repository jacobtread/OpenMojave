use super::{prelude::*, regn::REGN, soun::SOUN};
use crate::esp::record::sub::object_bounds::ObjectBounds;

/// Acoustic Space
///
/// Defines an area that contains sound
#[derive(Debug)]
pub struct ASPC {
    /// Object editor ID
    pub editor_id: EditorId,
    /// Bounds of the space object
    pub object_bounds: ObjectBounds,
    /// Nullable sound thats played by default within this space
    pub default_loop: NTypedFormId<SOUN>,
    /// Nullable sound played during the afternoon
    pub afternoon: NTypedFormId<SOUN>,
    /// Nullable sound played at dusk
    pub dusk: NTypedFormId<SOUN>,
    /// Nullable sound played at night
    pub night: NTypedFormId<SOUN>,
    /// Nullable crowd murmor background sound
    pub walla: NTypedFormId<SOUN>,
    /// The number of entities required to trigger the walla sound
    pub walla_trigger_count: u32,
    /// Optionally use sound from a region (Interiors only)
    pub use_region_sound: Option<TypedFormId<REGN>>,
    /// The type of environment
    pub env_type: EnvironmentType,
    /// Whether the space is an interior (Why is this an enum???)
    pub is_interior: IsInterior,
}

impl Record for ASPC {
    const TYPE: RecordType = ASPC;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let object_bounds: ObjectBounds = parser.parse(OBND)?;
        let default_loop: NTypedFormId<SOUN> = parser.parse(SNAM)?;
        let afternoon: NTypedFormId<SOUN> = parser.parse(SNAM)?;
        let dusk: NTypedFormId<SOUN> = parser.parse(SNAM)?;
        let night: NTypedFormId<SOUN> = parser.parse(SNAM)?;
        let walla: NTypedFormId<SOUN> = parser.parse(SNAM)?;
        let walla_trigger_count: u32 = parser.parse(WNAM)?;
        let use_region_sound: Option<TypedFormId<REGN>> = parser.try_parse(RDAT)?;
        let env_type: EnvironmentType = parser.parse(ANAM)?;
        let is_interior: IsInterior = parser.parse(INAM)?;

        Ok(Self {
            editor_id,
            object_bounds,
            default_loop,
            afternoon,
            dusk,
            night,
            walla,
            walla_trigger_count,
            use_region_sound,
            env_type,
            is_interior,
        })
    }
}

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u32)]
pub enum EnvironmentType {
    None = 0,
    Default = 1,
    Generic = 2,
    PaddedCell = 3,
    Room = 4,
    Bathroom = 5,
    Livingroom = 6,
    StoneRoom = 7,
    Auditorium = 8,
    Concerthall = 9,
    Cave = 10,
    Arena = 11,
    Hangar = 12,
    CarpetedHallway = 13,
    Hallway = 14,
    StoneCorridor = 15,
    Alley = 16,
    Forest = 17,
    City = 18,
    Mountains = 19,
    Quarry = 20,
    Plain = 21,
    Parkinglot = 22,
    Sewerpipe = 23,
    Underwater = 24,
    SmallRoom = 25,
    MediumRoom = 26,
    LargeRoom = 27,
    MediumHall = 28,
    LargeHall = 29,
    Plate = 30,
}

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u32)]
pub enum IsInterior {
    No = 1,
    Yes = 2,
}

impl FromRecordBytes for EnvironmentType {
    #[inline]
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        enum_value(input)
    }
}

impl FromRecordBytes for IsInterior {
    #[inline]
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        enum_value(input)
    }
}
