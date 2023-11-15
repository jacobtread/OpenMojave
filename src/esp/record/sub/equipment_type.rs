use crate::esp::record::{enum_value, FromRecordBytes};
use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(i32)]
pub enum EquipmentType {
    None = -1,
    BigGuns = 0,
    EnergyWeapons = 1,
    SmallGuns = 2,
    MeleeWeapons = 3,
    UnarmedWeapon = 4,
    ThrownWeapons = 5,
    Mine = 6,
    BodyWear = 7,
    HeadWear = 8,
    HandWear = 9,
    Chems = 10,
    Stimpack = 11,
    Food = 12,
    Alcohol = 13,
}

impl FromRecordBytes for EquipmentType {
    #[inline]
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        enum_value(input)
    }
}
