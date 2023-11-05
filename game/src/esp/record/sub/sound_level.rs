use num_enum::TryFromPrimitive;

use crate::esp::record::{enum_value, FromRecordBytes};

#[derive(Debug, Clone, Copy, TryFromPrimitive, PartialEq, Eq)]
#[repr(u32)]
pub enum SoundLevel {
    Loud = 0,
    Normal = 1,
    Silent = 2,
}

impl FromRecordBytes for SoundLevel {
    #[inline]
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        enum_value(input)
    }
}
