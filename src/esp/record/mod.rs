#![allow(clippy::upper_case_acronyms)]

use bitflags::bitflags;
use nalgebra::{Vector2, Vector3};
use nom::bytes::complete::take;
use nom::combinator::{all_consuming, map_res};
use nom::multi::many0;
use nom::number::complete::{i8, le_f32, le_i16, le_i32, le_u16, le_u32, u8};
use nom::sequence::tuple;
use sub::GRUP;

use self::records::cell::CELL;
use self::records::dial::DIAL;
use self::records::prelude::TypedFormId;
use self::records::wrld::WRLD;
use self::records::ParsedRecord;

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
use std::any::{type_name, Any};
use std::f32::consts::E;
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
#[inline]
pub fn take4(input: &[u8]) -> IResult<&[u8], [u8; 4]> {
    take_bytes_const(input)
}

/// Takes 4 bytes from the input returning them as a fixed length array
pub fn take_bytes_const<const C: usize>(input: &[u8]) -> IResult<&[u8], [u8; C]> {
    map_res(take(C), TryInto::try_into)(input)
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

#[derive(Debug)]
pub struct OwnedRawRecord {
    pub ty: RecordType,
    pub flags: RecordFlags,
    pub form_id: u32,
    pub revision: u32,
    pub version: u16,
    pub data: Vec<u8>,
}

impl<'b> RawRecord<'b> {
    #[inline]
    pub fn parsed<'a>(&'a self) -> Result<ParsedRecord, RecordParseError<'b>> {
        ParsedRecord::parse(self)
    }

    pub fn parse_record<'a, R: Record>(&'a self) -> Result<R, RecordParseError<'b>> {
        let mut parser = RecordParser::new(self)?;
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
    pub label: [u8; 4],
    pub ty: GroupType,
    pub stamp: u16,
    pub data: &'a [u8],
}

pub enum Group {
    TopLevel {
        label: RecordType,
        records: Vec<EsmEntry>,
    },
    WorldChildren {
        world: TypedFormId<WRLD>,
        records: Vec<EsmEntry>,
    },
    InteriorCellBlock {
        cell_block_number: i32,
        records: Vec<EsmEntry>,
    },
    InteriorCellSubBlock {
        cell_sub_block_number: i32,
        records: Vec<EsmEntry>,
    },
    ExteriorCellBlock {
        y: u8,
        x: u8,
        records: Vec<EsmEntry>,
    },
    ExteriorCellSubBlock {
        y: u8,
        x: u8,
        records: Vec<EsmEntry>,
    },
    CellChildren {
        cell: TypedFormId<CELL>,
        records: Vec<EsmEntry>,
    },
    TopicChildren {
        cell: TypedFormId<DIAL>,
        records: Vec<EsmEntry>,
    },
    CellPersistentChildren {
        cell: TypedFormId<CELL>,
        records: Vec<EsmEntry>,
    },
    CellTemporaryChildren {
        cell: TypedFormId<CELL>,
        records: Vec<EsmEntry>,
    },
    CellVisibleDistantChildren {
        cell: TypedFormId<CELL>,
        records: Vec<EsmEntry>,
    },
}

pub enum EsmEntry {
    Record(ParsedRecord),
    Group(Group),
}

impl<'b> RawGroup<'b> {
    const HEADER_LENGTH: u32 = 24;
    const GROUP_RECORD: RecordType = GRUP;

    pub fn parsed(&self) -> Result<Group, RecordParseError<'b>> {
        let records = RawEsmEntry::parsed_all(&self.data)?;

        Ok(match self.ty {
            GroupType::TopLevel => Group::TopLevel {
                label: RecordType(self.label),
                records,
            },
            GroupType::WorldChildren => Group::WorldChildren {
                world: FormId(u32::from_be_bytes(self.label)).into_typed(),
                records,
            },
            GroupType::InteriorCellBlock => Group::InteriorCellBlock {
                cell_block_number: i32::from_be_bytes(self.label),
                records,
            },
            GroupType::InteriorCellSubBlock => Group::InteriorCellSubBlock {
                cell_sub_block_number: i32::from_be_bytes(self.label),
                records,
            },
            GroupType::ExteriorCellBlock => Group::ExteriorCellBlock {
                y: self.label[0],
                x: self.label[1],
                records,
            },
            GroupType::ExteriorCellSubBlock => Group::ExteriorCellSubBlock {
                y: self.label[0],
                x: self.label[1],
                records,
            },
            GroupType::CellChildren => Group::CellChildren {
                cell: FormId(u32::from_be_bytes(self.label)).into_typed(),
                records,
            },
            GroupType::TopicChildren => Group::TopicChildren {
                cell: FormId(u32::from_be_bytes(self.label)).into_typed(),
                records,
            },
            GroupType::CellPersistentChildren => Group::CellPersistentChildren {
                cell: FormId(u32::from_be_bytes(self.label)).into_typed(),
                records,
            },
            GroupType::CellTemporaryChildren => Group::CellTemporaryChildren {
                cell: FormId(u32::from_be_bytes(self.label)).into_typed(),
                records,
            },
            GroupType::CellVisibleDistantChildren => Group::CellVisibleDistantChildren {
                cell: FormId(u32::from_be_bytes(self.label)).into_typed(),
                records,
            },
        })
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], RawGroup<'_>> {
        let (input, size) = le_u32(input)?;
        let (input, label) = take4(input)?;
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
    pub fn parse_inner(&self) -> IResult<&[u8], Vec<RawEsmEntry>> {
        RawEsmEntry::parse_all(self.data)
    }
}

/// Raw record within the file
pub enum RawEsmEntry<'a> {
    Record(RawRecord<'a>),
    Group(RawGroup<'a>),
}

impl<'b> RawEsmEntry<'b> {
    pub fn parsed(&self) -> Result<EsmEntry, RecordParseError<'b>> {
        match self {
            RawEsmEntry::Record(record) => record.parsed().map(EsmEntry::Record),
            RawEsmEntry::Group(group) => group.parsed().map(EsmEntry::Group),
        }
    }

    pub fn parsed_all(input: &[u8]) -> Result<Vec<EsmEntry>, RecordParseError<'_>> {
        let (_, raw_records) = Self::parse_all(input)?;

        let mut records: Vec<EsmEntry> = Vec::with_capacity(raw_records.len());
        for raw_record in raw_records {
            let record = match raw_record.parsed() {
                Ok(value) => value,
                Err(err) => {
                    // TODO: Err
                    continue;
                }
            };
            records.push(record)
        }

        Ok(records)
    }

    pub fn parse_all(input: &[u8]) -> IResult<&[u8], Vec<RawEsmEntry<'_>>> {
        all_consuming(many0(Self::parse))(input)
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], RawEsmEntry<'_>> {
        let (input, ty) = RecordType::parse(input)?;

        if ty == RawGroup::GROUP_RECORD {
            RawGroup::parse(input)
                // Convert into plugin entry
                .map(|(input, group)| (input, RawEsmEntry::Group(group)))
        } else {
            RawRecord::parse(input, ty)
                // Convert into plugin entry
                .map(|(input, group)| (input, RawEsmEntry::Record(group)))
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
    pub records: Vec<RawSubRecord<'b>>,
    pub record_index: usize,
}

impl<'a, 'b> RecordParser<'a, 'b> {
    pub fn new(record: &'a RawRecord<'b>) -> Result<RecordParser<'a, 'b>, RecordParseError<'b>> {
        let (_, records) = RawSubRecord::parse_all(record.data)?;

        Ok(RecordParser {
            record,
            records,
            record_index: 0,
        })
    }

    /// Gets the next element requiring that one exist
    fn next(&mut self) -> Result<&RawSubRecord<'b>, RecordParseError<'b>> {
        let value = self
            .records
            .get(self.record_index)
            .ok_or(RecordParseError::NoMoreContent)?;
        self.record_index += 1;
        Ok(value)
    }

    /// Gets the next element requiring that one exist and
    /// is of the provided type
    fn require_next_typed(
        &mut self,
        ty: RecordType,
    ) -> Result<&RawSubRecord<'b>, RecordParseError<'b>> {
        self.next()
            // Error if the record type doesn't match
            .and_then(|record| {
                if record.ty == ty {
                    Ok(record)
                } else {
                    Err(RecordParseError::UnexpectedType {
                        expected: ty,
                        actual: record.ty,
                    })
                }
            })
    }

    /// Gets the next record if it matches the provided type
    pub fn next_if(&mut self, ty: RecordType) -> Option<&RawSubRecord<'b>> {
        let item = self.records.get(self.record_index)?;
        if item.ty == ty {
            self.record_index += 1;
            Some(item)
        } else {
            None
        }
    }

    /// Requires the next record type is the the provided type
    ///
    /// Usually used for ensuring marker types exist
    pub fn require_type(&mut self, ty: RecordType) -> Result<(), RecordParseError<'b>> {
        _ = self.require_next_typed(ty)?;
        Ok(())
    }

    /// Skips the next type ignoring whether it exists
    #[inline]
    pub fn skip_type(&mut self, ty: RecordType) {
        _ = self.next_if(ty);
    }

    pub fn skip_while_type(&mut self, ty: RecordType) {
        while self.next_if(ty).is_some() {}
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
        self.next_if(ty)
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

pub trait Record: Sized + Send + Sync {
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

    let (input, header) = RawEsmEntry::parse(&bytes).unwrap();

    let header = match header {
        RawEsmEntry::Record(record) => record.parse_record::<TES4>(),
        RawEsmEntry::Group(_) => panic!("Expected first entry to be a header"),
    }
    .unwrap();

    dbg!(&header);

    let (_, records): (&[u8], Vec<RawEsmEntry>) = RawEsmEntry::parse_all(&input).unwrap();

    let scripts = records
        .iter()
        .filter_map(|value| match value {
            RawEsmEntry::Record(_) => None,
            RawEsmEntry::Group(group) => {
                if group.ty == GroupType::TopLevel && group.label == RecordType::new(b"SCPT").0 {
                    Some(group.data)
                } else {
                    None
                }
            }
        })
        .next()
        .unwrap();

    let scripts: Vec<SCPT> = RawEsmEntry::parse_all(scripts)
        .unwrap()
        .1
        .into_iter()
        .filter_map(|value| match value {
            RawEsmEntry::Record(record) => Some(record.parse_record::<SCPT>().unwrap()),
            RawEsmEntry::Group(group) => None,
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
