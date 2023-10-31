use bitflags::bitflags;
use nom::{
    bytes::complete::take,
    combinator::{all_consuming, map, map_parser},
    multi::many0,
    number::complete::{le_u16, le_u32},
    IResult,
};
use std::fmt::{Debug, Display};

bitflags! {
    #[derive(Debug, Clone)]
    pub struct RecordFlags: u32 {
        /// The plugin is a master file.
        const MASTER = 0x00000001;
        /// Record is compressed
        const COMPRESSED = 0x00040000;
    }
}

impl RecordFlags {
    /// Parses record flags from the provided input
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(le_u32, RecordFlags::from_bits_retain)(input)
    }
}

/// Record type identifier
#[derive(Hash, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct RecordType([u8; 4]);

impl RecordType {
    /// Create a record type from a 4 byte string
    pub const fn from_value(&[a, b, c, d]: &[u8; 4]) -> Self {
        // String comes in big endian byte order
        Self([a, b, c, d])
    }

    /// Get the string representation of the record type
    pub const fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }

    /// Parse a record from the provided input
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(take(4usize), |bytes: &[u8]| {
            RecordType(
                bytes
                    .try_into()
                    .expect("Nom provided incorrect length of bytes"),
            )
        })(input)
    }
}

/// Record type within a file
pub struct RawRecord<'a> {
    pub ty: RecordType,
    pub flags: RecordFlags,
    pub form_id: u32,
    pub revision: u32,
    pub version: u16,
    pub data: &'a [u8],
}

impl RawRecord<'_> {
    pub fn parse(input: &[u8], ty: RecordType) -> IResult<&[u8], RawRecord<'_>> {
        let (input, size) = le_u32(input)?;
        let (input, flags) = RecordFlags::parse(input)?;
        let (input, form_id) = le_u32(input)?;
        let (input, revision) = le_u32(input)?;
        let (input, version) = le_u16(input)?;
        let (input, _unknown) = le_u16(input)?;
        let (input, data) = take(size)(input)?;

        Ok((
            input,
            RawRecord {
                ty,
                flags,
                form_id,
                revision,
                version,
                data,
            },
        ))
    }
}

pub struct RawGroup<'a> {
    pub label: u32,
    pub ty: RecordType,
    pub stamp: u16,
    pub data: &'a [u8],
}

impl RawGroup<'_> {
    const HEADER_LENGTH: u32 = 24;
    const GROUP_RECORD: RecordType = RecordType::from_value(b"GRUP");

    pub fn parse(input: &[u8]) -> IResult<&[u8], RawGroup<'_>> {
        let (input, size) = le_u32(input)?;
        let (input, label) = le_u32(input)?;
        let (input, ty) = RecordType::parse(input)?;
        let (input, stamp) = le_u16(input)?;
        let (input, _unknown) = take(6usize)(input)?;
        let (input, data) = take(size - Self::HEADER_LENGTH)(input)?;

        Ok((
            input,
            RawGroup {
                label,
                ty,
                stamp,
                data,
            },
        ))
    }
}

/// Raw record within the file
pub enum PluginEntry<'a> {
    Record(RawRecord<'a>),
    Group(RawGroup<'a>),
}

impl PluginEntry<'_> {
    pub fn parse_all(input: &[u8]) -> IResult<&[u8], Vec<PluginEntry<'_>>> {
        all_consuming(many0(Self::parse))(input)
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], PluginEntry<'_>> {
        let (input, ty) = RecordType::parse(input)?;

        if ty == RawGroup::GROUP_RECORD {
            RawGroup::parse(input)
                // Convert into plugin entry
                .map(|(input, group)| (input, PluginEntry::Group(group)))
        } else {
            RawRecord::parse(input, ty)
                // Convert into plugin entry
                .map(|(input, group)| (input, PluginEntry::Record(group)))
        }
    }
}

/// Display record types as strings
impl Display for RecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Display record types as strings
impl Debug for RecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[test]
fn test_parse() {
    let bytes = std::fs::read("../Data/FalloutNV.esm").unwrap();
    let (_, records): (&[u8], Vec<PluginEntry>) = PluginEntry::parse_all(&bytes).unwrap();

    println!("Parsed: {}", records.len());
}
