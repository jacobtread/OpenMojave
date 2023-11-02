use num_enum::TryFromPrimitive;

use crate::esp::{
    record::{
        enum_value,
        sub::{object_bounds::ObjectBounds, ANAM, EDID, INAM, OBND, RDAT, SNAM, WNAM},
        FromRecordBytes, Record, RecordParseError, RecordParser, RecordType,
    },
    shared::{EditorId, FormId, TypedFormId},
};

use super::soun::SOUN;

/// Represents a sound record
type SoundRecord = TypedFormId<SOUN>;

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
    pub default_loop: SoundRecord,
    /// Nullable sound played during the afternoon
    pub afternoon: SoundRecord,
    /// Nullable sound played at dusk
    pub dusk: SoundRecord,
    /// Nullable sound played at night
    pub night: SoundRecord,
    /// Nullable crowd murmor background sound
    pub walla: SoundRecord,
    /// The number of entities required to trigger the walla sound
    pub walla_trigger_count: u32,
    /// Optionally use sound from a region (Nullable) (Interiors only)
    pub use_region_sound: Option<FormId /* REGN */>,
    /// The type of environment
    pub env_type: EnvironmentType,
    /// Whether the space is an interior (Why is this an enum???)
    pub is_interior: IsInterior,
}

impl Record for ASPC {
    const TYPE: RecordType = RecordType::from_value(b"ASPC");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id = parser.parse::<EditorId>(EDID)?;
        let object_bounds = parser.parse::<ObjectBounds>(OBND)?;
        let default_loop = parser.parse::<SoundRecord>(SNAM)?;
        let afternoon = parser.parse::<SoundRecord>(SNAM)?;
        let dusk = parser.parse::<SoundRecord>(SNAM)?;
        let night = parser.parse::<SoundRecord>(SNAM)?;
        let walla = parser.parse::<SoundRecord>(SNAM)?;
        let walla_trigger_count = parser.parse::<u32>(WNAM)?;
        let use_region_sound = parser.try_parse::<FormId>(RDAT)?;
        let env_type = parser.parse::<EnvironmentType>(ANAM)?;
        let is_interior = parser.parse::<IsInterior>(INAM)?;
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

impl FromRecordBytes for EnvironmentType {
    #[inline]
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        enum_value(input)
    }
}

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u32)]
pub enum IsInterior {
    No = 1,
    Yes = 2,
}

impl FromRecordBytes for IsInterior {
    #[inline]
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        enum_value(input)
    }
}
