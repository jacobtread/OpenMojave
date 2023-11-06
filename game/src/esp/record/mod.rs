#![allow(clippy::upper_case_acronyms)]

use bitflags::bitflags;
use fyrox::core::algebra::{Vector2, Vector3};
use nom::bytes::complete::take;
use nom::combinator::{all_consuming, map_res};
use nom::multi::many0;
use nom::number::complete::{i8, le_f32, le_i16, le_i32, le_u16, le_u32, u8};
use nom::sequence::tuple;

use super::shared::FormId;
use crate::esp::record::records::scpt::SCPT;
use crate::esp::record::records::tes4::TES4;
use nom::Parser;
use nom::{
    bytes::complete::{tag, take_while},
    combinator::{map, rest},
    sequence::terminated,
    IResult,
};
use num_enum::TryFromPrimitive;
use std::any::type_name;
use std::path::Path;
use std::{
    fmt::{Debug, Display},
    iter::Peekable,
};
use thiserror::Error;

pub mod records;
pub mod sub;

pub fn parse_string(input: &[u8]) -> IResult<&[u8], String> {
    // TODO: Possibly use CString instead
    map(
        terminated(
            // While non null
            take_while(|byte: u8| byte != 0),
            // Null terminator tag
            tag("\0"),
        ),
        |bytes| String::from_utf8_lossy(bytes).to_string(),
    )(input)
}

/// Takes 4 bytes from the input returning them as a fixed length array
pub fn take4(input: &[u8]) -> IResult<&[u8], [u8; 4]> {
    map_res(take(4usize), TryInto::try_into)(input)
}

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
#[derive(Hash, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct RecordType([u8; 4]);

impl RecordType {
    /// Create a record type from a 4 byte string
    pub const fn new(&[a, b, c, d]: &[u8; 4]) -> Self {
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
#[derive(Debug)]
pub struct RawRecord<'a> {
    pub ty: RecordType,
    pub flags: RecordFlags,
    pub form_id: u32,
    pub revision: u32,
    pub version: u16,
    pub data: &'a [u8],
}

impl<'b> RawRecord<'b> {
    pub fn parse_record<'a, R: Record>(&'a self) -> Result<R, RecordParseError<'b>> {
        let (_, records) = RawSubRecord::parse_all(self.data)?;
        println!("Total Records: {}", records.len());

        let mut parser = RecordParser {
            record: self,
            record_iter: records.iter().peekable(),
        };

        R::parse(&mut parser)
    }

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

    /// Parses the inner esm entries of this group
    pub fn parse_inner(&self) -> IResult<&[u8], Vec<RawSubRecord<'_>>> {
        RawSubRecord::parse_all(self.data)
    }
}

#[derive(Debug)]
pub struct RawSubRecord<'a> {
    pub ty: RecordType,
    pub data: &'a [u8],
}

impl RawSubRecord<'_> {
    pub fn parse_all(input: &[u8]) -> IResult<&[u8], Vec<RawSubRecord<'_>>> {
        all_consuming(many0(Self::parse))(input)
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], RawSubRecord<'_>> {
        let (input, ty) = RecordType::parse(input)?;
        let (input, size) = le_u16(input)?;
        let (input, data) = take(size)(input)?;

        Ok((input, RawSubRecord { ty, data }))
    }
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum GroupType {
    TopLevel = 0,
    WorldChildren = 1,
    InteriorCellBlock = 2,
    InteriorCellSubBlock = 3,
    ExteriorCellBlock = 4,
    ExteriorCellSubBlock = 5,
    CellChildren = 6,
    TopicChildren = 7,
    CellPersistentChildren = 8,
    CellTemporaryChildren = 9,
    CellVisibleDistantChildren = 10,
}

/// Parses an enum from a little endian u32 value
pub fn enum_value<E>(input: &[u8]) -> IResult<&[u8], E>
where
    E: TryFromPrimitive,
    E::Primitive: FromRecordBytes,
{
    map_res(E::Primitive::parse, E::try_from_primitive)(input)
}

#[derive(Debug)]
pub struct RawGroup<'a> {
    pub label: u32,
    pub ty: GroupType,
    pub stamp: u16,
    pub data: &'a [u8],
}

pub enum TypedGroup<'a> {
    TopLevel {
        label: RecordType,
        records: RawRecord<'a>,
    },
    WorldChildren {
        world: FormId,
        records: RawRecord<'a>,
    },
    InteriorCellBlock {
        cell_block_number: i32,
        records: RawRecord<'a>,
    },
    InteriorCellSubBlock {
        cell_sub_block_number: i32,
        records: RawRecord<'a>,
    },
    ExteriorCellBlock {
        y: u8,
        x: u8,
        records: RawRecord<'a>,
    },
    ExteriorCellSubBlock {
        y: u8,
        x: u8,
        records: RawRecord<'a>,
    },
    CellChildren {
        cell: FormId,
        records: RawRecord<'a>,
    },
    TopicChildren {
        cell: FormId,
        records: RawRecord<'a>,
    },
    CellPersistentChildren {
        cell: FormId,
        records: RawRecord<'a>,
    },
    CellTemporaryChildren {
        cell: FormId,
        records: RawRecord<'a>,
    },
    CellVisibleDistantChildren {
        cell: FormId,
        records: RawRecord<'a>,
    },
}

impl RawGroup<'_> {
    const HEADER_LENGTH: u32 = 24;
    const GROUP_RECORD: RecordType = RecordType::new(b"GRUP");

    pub fn parse(input: &[u8]) -> IResult<&[u8], RawGroup<'_>> {
        let (input, size) = le_u32(input)?;
        let (input, label) = le_u32(input)?;
        let (input, ty) = enum_value::<GroupType>(input)?;
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

    /// Parses the inner esm entries of this group
    pub fn parse_inner(&self) -> IResult<&[u8], Vec<EsmEntry>> {
        EsmEntry::parse_all(self.data)
    }
}

/// Raw record within the file
pub enum EsmEntry<'a> {
    Record(RawRecord<'a>),
    Group(RawGroup<'a>),
}

impl EsmEntry<'_> {
    pub fn parse_all(input: &[u8]) -> IResult<&[u8], Vec<EsmEntry<'_>>> {
        all_consuming(many0(Self::parse))(input)
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], EsmEntry<'_>> {
        let (input, ty) = RecordType::parse(input)?;

        if ty == RawGroup::GROUP_RECORD {
            RawGroup::parse(input)
                // Convert into plugin entry
                .map(|(input, group)| (input, EsmEntry::Group(group)))
        } else {
            RawRecord::parse(input, ty)
                // Convert into plugin entry
                .map(|(input, group)| (input, EsmEntry::Record(group)))
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

/// Trait for types that can be extracted from a sub record
pub trait FromRecordBytes: Sized {
    fn parse(input: &[u8]) -> IResult<&[u8], Self>;
}

/// FromRecordBytes implementor for getting all the bytes
#[derive(Debug)]
pub struct RawBytes(pub Vec<u8>);

impl RawBytes {
    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}

impl FromRecordBytes for RawBytes {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(rest, |bytes: &[u8]| Self(bytes.to_vec()))(input)
    }
}

/// String created from the entirety of the sub record bytes
pub struct FullString(pub String);

impl FromRecordBytes for FullString {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(rest, |bytes: &[u8]| {
            Self(String::from_utf8_lossy(bytes).to_string())
        })(input)
    }
}

impl FullString {
    pub fn into_inner(self) -> String {
        self.0
    }
}

/// Zero or more repeated values from a sub-record
pub struct Repeated<T: FromRecordBytes>(pub Vec<T>);

impl<T> FromRecordBytes for Repeated<T>
where
    T: FromRecordBytes,
{
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(many0(T::parse), Self)(input)
    }
}

impl<T> Repeated<T>
where
    T: FromRecordBytes,
{
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }
}

pub trait RecordCollection: Sized {
    /// Attempts to parse the next item for this collection if present
    fn parse_next<'b>(
        parser: &mut RecordParser<'_, 'b>,
    ) -> Result<Option<Self>, RecordParseError<'b>>;

    /// Wrapper around `parse_next` requiring that a value is returned
    fn require_parse_next<'b>(
        parser: &mut RecordParser<'_, 'b>,
    ) -> Result<Self, RecordParseError<'b>> {
        Self::parse_next(parser)?.ok_or_else(|| {
            let msg = format!("Missing expected {} collection", type_name::<Self>());
            RecordParseError::Custom(msg)
        })
    }
}

/// Iterator over sub records to make parsing easier
pub struct RecordParser<'a, 'b> {
    pub record: &'a RawRecord<'b>,
    /// Iterator over the raw records
    pub record_iter: Peekable<std::slice::Iter<'a, RawSubRecord<'b>>>,
}

impl<'a, 'b> RecordParser<'a, 'b> {
    /// Gets the next element requiring that one exist
    fn require_next(&mut self) -> Result<&RawSubRecord<'b>, RecordParseError<'b>> {
        self.record_iter
            .next()
            .ok_or(RecordParseError::NoMoreContent)
    }

    /// Gets the next element requiring that one exist and
    /// is of the provided type
    fn require_next_typed(
        &mut self,
        ty: RecordType,
    ) -> Result<&RawSubRecord<'b>, RecordParseError<'b>> {
        let record = self.require_next()?;

        if record.ty != ty {
            return Err(RecordParseError::UnexpectedType {
                expected: ty,
                actual: record.ty,
            });
        }

        Ok(record)
    }

    /// Gets the next record if it matches the provided type
    pub fn try_next(&mut self, ty: RecordType) -> Option<&RawSubRecord<'b>> {
        self.record_iter.next_if(|record| record.ty == ty)
    }

    /// Requires the next record type is the the provided type
    ///
    /// Usually used for ensuring marker types exist
    pub fn require_type(&mut self, ty: RecordType) -> Result<(), RecordParseError<'b>> {
        _ = self.require_next_typed(ty)?;
        Ok(())
    }

    /// Skips the next type ignoring whether it exists
    pub fn skip_type(&mut self, ty: RecordType) {
        self.record_iter.next_if(|record| record.ty == ty);
    }

    pub fn skip_while_type(&mut self, ty: RecordType) {
        while self.record_iter.next_if(|record| record.ty == ty).is_some() {}
    }

    pub fn parse<T>(&mut self, ty: RecordType) -> Result<T, RecordParseError<'b>>
    where
        T: FromRecordBytes,
    {
        // Require the next record of the type
        self.require_next_typed(ty)
            // Parse the contents of the record
            .and_then(|record| {
                let (_, this) = all_consuming(T::parse)(record.data)?;
                Ok(this)
            })
    }

    pub fn parse_collection<T>(&mut self) -> Result<Vec<T>, RecordParseError<'b>>
    where
        T: RecordCollection,
    {
        let mut out = Vec::new();

        while let Some(value) = T::parse_next(self)? {
            out.push(value);
        }

        Ok(out)
    }

    pub fn try_parse<T>(&mut self, ty: RecordType) -> Result<Option<T>, RecordParseError<'b>>
    where
        T: FromRecordBytes,
    {
        self.record_iter
            // Only take next record if the type matches
            .next_if(|record| record.ty == ty)
            // Attempt to parse the matching record
            .map(|record| {
                let (_, this) = all_consuming(T::parse)(record.data)?;
                Ok(this)
            })
            // Flip the option to inside the result
            .transpose()
    }

    /// Attempts to parse as many of the provided type as it can
    /// storing them in a vec
    pub fn try_parse_many<T>(&mut self, ty: RecordType) -> Result<Vec<T>, RecordParseError<'b>>
    where
        T: FromRecordBytes,
    {
        let mut out = Vec::new();
        while let Some(value) = self.try_parse::<T>(ty)? {
            out.push(value);
        }
        Ok(out)
    }
}

pub trait Record: Sized {
    const TYPE: RecordType;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>>;
}

#[derive(Debug, Error)]
pub enum RecordParseError<'a> {
    /// Error from nom while parsing
    #[error(transparent)]
    Nom(nom::Err<nom::error::Error<&'a [u8]>>),
    /// Reached an unexpected record type
    #[error("Unexpected record type {expected} {actual}")]
    UnexpectedType {
        /// The expected record type
        expected: RecordType,
        /// The actual record type
        actual: RecordType,
    },
    /// Tried to read another sub-record but there wasn't one
    #[error("No more sub records to read")]
    NoMoreContent,
    /// Custom string error message
    #[error("{0}")]
    Custom(String),
}

impl<'a> From<nom::Err<nom::error::Error<&'a [u8]>>> for RecordParseError<'a> {
    fn from(value: nom::Err<nom::error::Error<&'a [u8]>>) -> Self {
        Self::Nom(value)
    }
}

impl FromRecordBytes for String {
    #[inline]
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        parse_string(input)
    }
}

impl FromRecordBytes for f32 {
    #[inline]
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        le_f32(input)
    }
}

impl FromRecordBytes for i32 {
    #[inline]
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        le_i32(input)
    }
}

impl FromRecordBytes for u32 {
    #[inline]
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        le_u32(input)
    }
}

impl FromRecordBytes for i16 {
    #[inline]
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        le_i16(input)
    }
}

impl FromRecordBytes for u16 {
    #[inline]
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        le_u16(input)
    }
}

impl FromRecordBytes for u8 {
    #[inline]
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        u8(input)
    }
}

impl FromRecordBytes for i8 {
    #[inline]
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        i8(input)
    }
}
impl FromRecordBytes for bool {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(u8, |value| value != 0)(input)
    }
}

impl<T> FromRecordBytes for Vector3<T>
where
    T: FromRecordBytes,
{
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(tuple((T::parse, T::parse, T::parse)), |(a, b, c)| {
            Vector3::new(a, b, c)
        })(input)
    }
}
impl<T> FromRecordBytes for Vector2<T>
where
    T: FromRecordBytes,
{
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(tuple((T::parse, T::parse)), |(a, b)| Vector2::new(a, b))(input)
    }
}

#[test]
fn test_parse() {
    let bytes = std::fs::read("../Data/FalloutNV.esm").unwrap();

    let (input, header) = EsmEntry::parse(&bytes).unwrap();

    let header = match header {
        EsmEntry::Record(record) => record.parse_record::<TES4>(),
        EsmEntry::Group(_) => panic!("Expected first entry to be a header"),
    }
    .unwrap();

    dbg!(&header);

    let (_, records): (&[u8], Vec<EsmEntry>) = EsmEntry::parse_all(&input).unwrap();

    let scripts = records
        .iter()
        .filter_map(|value| match value {
            EsmEntry::Record(_) => None,
            EsmEntry::Group(group) => {
                if group.ty == GroupType::TopLevel
                    && group.label.to_le_bytes() == RecordType::new(b"SCPT").0
                {
                    Some(group.data)
                } else {
                    None
                }
            }
        })
        .next()
        .unwrap();

    let scripts: Vec<SCPT> = EsmEntry::parse_all(scripts)
        .unwrap()
        .1
        .into_iter()
        .filter_map(|value| match value {
            EsmEntry::Record(record) => Some(record.parse_record::<SCPT>().unwrap()),
            EsmEntry::Group(group) => None,
        })
        .collect();

    for script in scripts {
        let name = script.editor_id;
        let script = script.script;
        let source = script.source;
        std::fs::write(format!("../DataUnpacked/Scripts/{}.script", name.0), source).unwrap();
    }

    println!("Parsed: {}", records.len());

    // let gmst_group = records.iter()
    // .find_map(|value| match value {
    //     EsmEntry::Record(_) => None,
    //     EsmEntry::Group(group) => {
    //         if group.ty == GroupType::TopLevel {

    //         }
    //     }
    // })

    // for record in records {
    //     match record {
    //         EsmEntry::Record(record) => {
    //             println!("{}", record.ty)
    //         }
    //         EsmEntry::Group(group) => {
    //             let (_, records) = group.parse_inner().unwrap();
    //             println!("{:?}", group.ty);
    //             println!("Parsed: {}", records.len());
    //         }
    //     }
    // }
}
