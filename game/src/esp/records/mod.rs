#![allow(clippy::upper_case_acronyms)]

use std::{
    fmt::{Debug, Display},
    fs::File,
    io::Cursor,
};

use bitflags::bitflags;
use nom::{
    bytes::complete::*, combinator::*, multi::many0, number::complete::*, sequence::tuple, IResult,
    Parser,
};

use crate::esp::records::tes4::parse_tex4;

pub mod record;
pub mod sub;
pub mod tes4;

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

#[derive(Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct RecordType(u32);

pub const fn record_type(value: &[u8; 4]) -> RecordType {
    RecordType(u32::from_le_bytes(*value))
}

impl Display for RecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(unsafe { std::str::from_utf8_unchecked(&self.0.to_le_bytes()) })
    }
}
impl Debug for RecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(unsafe { std::str::from_utf8_unchecked(&self.0.to_le_bytes()) })
    }
}

#[derive(Debug)]
pub struct RawRecord<'a> {
    pub ty: RecordType,
    pub flags: RecordFlags,
    pub form_id: u32,
    pub revision: u32,
    pub version: u16,
    pub unknown: u16,
    pub data: &'a [u8],
}

impl<'a> RawRecord<'a> {
    pub fn parse<F, O>(&self, mut parser: F) -> ParseResult<'_, O>
    where
        F: Parser<&'a [u8], O, nom::error::Error<&'a [u8]>>,
    {
        // todo: all-consuming
        let (_, o1) = parser.parse(self.data)?;

        Ok(o1)
    }
}

#[derive(Debug)]
pub struct RawSubRecord<'a> {
    pub ty: RecordType,
    pub data: &'a [u8],
}

#[derive(Debug)]
pub struct RawGroupRecord<'a> {
    pub label: u32,
    pub ty: RecordType,
    pub stamp: u16,
    pub unknown: [u8; 6],
    pub data: &'a [u8],
}

#[derive(Debug)]
pub enum AEntry<'a> {
    Record(RawRecord<'a>),
    Group(RawGroupRecord<'a>),
}

/// Parse a record type from the provided input
fn parse_record_type(input: &[u8]) -> IResult<&[u8], RecordType> {
    map(le_u32, RecordType)(input)
}

fn parse_record_flags(input: &[u8]) -> IResult<&[u8], RecordFlags> {
    map(le_u32, RecordFlags::from_bits_retain)(input)
}

fn parse_group_record(input: &[u8]) -> IResult<&[u8], AEntry<'_>> {
    let (input, size) = le_u32(input)?;
    let (input, label) = le_u32(input)?;
    let (input, ty) = parse_record_type(input)?;
    let (input, stamp) = le_u16(input)?;
    let (input, unknown) = take(6usize)(input)?;
    let (input, data) = take(size - 24)(input)?;

    Ok((
        input,
        AEntry::Group(RawGroupRecord {
            label,
            ty,
            stamp,
            unknown: unknown.try_into().unwrap(),
            data,
        }),
    ))
}

fn parse_normal_record(input: &[u8], ty: RecordType) -> IResult<&[u8], AEntry<'_>> {
    let (input, size) = le_u32(input)?;
    let (input, flags) = parse_record_flags(input)?;
    let (input, form_id) = le_u32(input)?;
    let (input, revision) = le_u32(input)?;
    let (input, version) = le_u16(input)?;
    let (input, unknown) = le_u16(input)?;
    let (input, data) = take(size)(input)?;

    Ok((
        input,
        AEntry::Record(RawRecord {
            ty,
            flags,
            form_id,
            revision,
            version,
            unknown,
            data,
        }),
    ))
}

fn parse_raw_record(input: &[u8]) -> IResult<&[u8], AEntry<'_>> {
    let (input, ty) = parse_record_type(input)?;

    println!("{}", ty);

    if ty == GROUP_RECORD {
        parse_group_record(input)
    } else {
        parse_normal_record(input, ty)
    }
}

const GROUP_RECORD: RecordType = record_type(b"GRUP");

pub type ParseResult<'a, V> = Result<V, nom::Err<nom::error::Error<&'a [u8]>>>;

fn parse_raw_sub_record(input: &[u8]) -> IResult<&[u8], RawSubRecord<'_>> {
    let (input, ty) = parse_record_type(input)?;
    let (input, size) = le_u16(input)?;
    let (input, data) = take(size)(input)?;

    Ok((input, RawSubRecord { ty, data }))
}

fn parse_records(input: &[u8]) -> Result<Vec<AEntry<'_>>, nom::Err<nom::error::Error<&[u8]>>> {
    let (remaining, records) = all_consuming(many0(parse_raw_record))(input)?;

    Ok(records)
}

#[test]
fn test() {
    let bytes = std::fs::read("../Data/FalloutNV.esm").unwrap();

    println!("Total: {}", bytes.len());

    // let (input, header) = parse_raw_record(&bytes).unwrap();
    // dbg!(&header);

    // let ha

    // let header = header.parse(parse_tex4).unwrap();

    // dbg!(&header);

    let records = match parse_records(&bytes) {
        Ok(value) => value,
        Err(err) => {
            match err {
                nom::Err::Incomplete(err) => {
                    println!("Incom: {:?}", err)
                }
                nom::Err::Error(err) => {
                    println!("Err: {:?}", err.code)
                }
                nom::Err::Failure(err) => {
                    println!("Fail: {:?}", err.code)
                }
            }

            panic!();
        }
    };

    println!("Result: {}", records.len())
}
