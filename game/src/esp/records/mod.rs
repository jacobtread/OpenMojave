#![allow(clippy::upper_case_acronyms)]

use std::{
    fmt::{Debug, Display},
    fs::File,
    io::Cursor,
};

use binrw::{binread, binrw, until_eof, BinRead};
use bitflags::bitflags;
use fyrox::core::num_traits::ToBytes;
use nom::{
    bytes::complete::*,
    combinator::*,
    multi::many0,
    number::{complete::*, streaming::le_u32},
    IResult, Parser,
};

use crate::esp::records::tes4::parse_tex4;

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

#[binrw]
#[brw(little)]
#[br(import(localized: bool))]
#[derive(Debug, Clone)]
pub enum Record {
    TES4(),
}

#[derive(Clone)]
#[repr(transparent)]
pub struct RecordType(u32);

pub const fn record_type(value: &[u8; 4]) -> RecordType {
    RecordType(u32::from_be_bytes(*value))
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
    pub timestamp: u16,
    pub version_control: u16,
    pub internal_version: u16,
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

/// Parse a record type from the provided input
fn parse_record_type(input: &[u8]) -> IResult<&[u8], RecordType> {
    map(le_u32, RecordType)(input)
}

fn parse_record_flags(input: &[u8]) -> IResult<&[u8], RecordFlags> {
    map(le_u32, RecordFlags::from_bits_retain)(input)
}

fn parse_raw_record(input: &[u8]) -> IResult<&[u8], RawRecord<'_>> {
    let (input, ty) = parse_record_type(input)?;
    let (input, size) = le_u32(input)?;
    let (input, flags) = parse_record_flags(input)?;
    let (input, form_id) = le_u32(input)?;
    let (input, timestamp) = le_u16(input)?;
    let (input, version_control) = le_u16(input)?;
    let (input, internal_version) = le_u16(input)?;
    let (input, unknown) = le_u16(input)?;

    let (input, data) = cut(take(size))(input)?;

    // Parse the sub records
    // let (_, records) = all_consuming(many0(raw_sub_record))(data).unwrap();

    // let records = vec![];

    Ok((
        input,
        RawRecord {
            ty,
            flags,
            form_id,
            timestamp,
            version_control,
            internal_version,
            unknown,
            data,
        },
    ))
}

pub type ParseResult<'a, V> = Result<V, nom::Err<nom::error::Error<&'a [u8]>>>;

fn parse_raw_sub_record(input: &[u8]) -> IResult<&[u8], RawSubRecord<'_>> {
    let (input, ty) = parse_record_type(input)?;
    let (input, size) = le_u16(input)?;
    let (input, data) = take(size)(input)?;

    Ok((input, RawSubRecord { ty, data }))
}

fn parse_records(input: &[u8]) -> Result<Vec<RawRecord<'_>>, nom::Err<nom::error::Error<&[u8]>>> {
    let (remaining, records) = many0(parse_raw_record)(input)?;
    println!("Remaining: {} {:?}", remaining.len(), &remaining[0..4]);

    Ok(records)
}

#[test]
fn test() {
    let bytes = std::fs::read("../Data/FalloutNV.esm").unwrap();

    let (input, header) = parse_raw_record(&bytes).unwrap();
    dbg!(&header);

    let header = header.parse(parse_tex4).unwrap();

    dbg!(&header);

    // let records = match parse_records(input) {
    //     Ok(value) => value,
    //     Err(err) => {
    //         match err {
    //             nom::Err::Incomplete(err) => {
    //                 println!("{:?}", err)
    //             }
    //             nom::Err::Error(err) => {
    //                 println!("{:?}", err.code)
    //             }
    //             nom::Err::Failure(err) => {
    //                 println!("{:?}", err.code)
    //             }
    //         }

    //         panic!();
    //     }
    // };

    // println!("Result: {}", records.len())
}
