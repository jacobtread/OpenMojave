use bitflags::bitflags;
use nom::{combinator::map, number::complete::le_u32, IResult};
use std::fmt::{Debug, Display};

bitflags! {
    #[derive(Debug, Clone)]
    pub struct RecordFlags: u32 {
        const MASTER = 0x00000001;
        const DELETED_GROUP = 0x00000010;
        const DELETED_RECORD = 0x00000010;
        const LOCALIZED = 0x00000080;
        const LIGHT_MASTER = 0x00000200;
        const COMPRESSED = 0x00040000;
    }
}

/// Record type identifier
#[derive(Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct RecordType(u32);

impl RecordType {
    /// Create a record type from a 4 byte string
    pub const fn from_value(value: &[u8; 4]) -> Self {
        Self(u32::from_le_bytes(*value))
    }

    /// Get the string representation of the record type
    pub const fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.0.to_le_bytes()) }
    }

    /// Parse a record from the provided input
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(le_u32, RecordType)(input)
    }
}

/// Record type within a file
pub struct RawRecord {}

/// Raw record within the file
pub enum RawRecord<'a> {}

/// Display record types as strings
impl Display for RecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Display record types as strings
impl Debug for RecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(self.as_str())
    }
}
