use bitflags::bitflags;
use nom::{
    bytes::complete::take,
    combinator::map,
    number::complete::{le_u16, le_u32, u8},
    sequence::tuple,
    IResult,
};
use num_enum::TryFromPrimitive;

use crate::esp::record::{enum_value, FromRecordBytes, FullString, RawBytes, RecordCollection};

use super::{SCDA, SCHR, SCRO, SCTX, SCVR, SLSD};

#[derive(Debug)]
pub struct Script {
    pub basic_data: SCHR,
    pub compiled_source: Vec<u8>,
    pub source: String,
    pub local_variables: Vec<LocalVariable>,
    pub references: Vec<Reference>,
}

impl RecordCollection for Script {
    fn parse_next<'b>(
        parser: &mut crate::esp::record::RecordParser<'_, 'b>,
    ) -> Result<Option<Self>, crate::esp::record::RecordParseError<'b>> {
        let basic_data = match parser.try_parse::<SCHR>(SCHR)? {
            Some(value) => value,
            None => return Ok(None),
        };
        let compiled_source = parser.parse::<RawBytes>(SCDA)?.into_inner();
        let source = parser.parse::<FullString>(SCTX)?.into_inner();
        let local_variables = parser.parse_collection::<LocalVariable>()?;
        let references = parser.try_parse_many::<Reference>(SCRO)?;
        Ok(Some(Self {
            basic_data,
            compiled_source,
            source,
            local_variables,
            references,
        }))
    }
}

#[derive(Debug)]
pub struct Reference(u32);

impl FromRecordBytes for Reference {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(le_u32, Reference)(input)
    }
}

#[derive(Debug)]
pub struct LocalVariable {
    pub data: SLSD,
    pub name: String,
}

impl RecordCollection for LocalVariable {
    fn parse_next<'b>(
        parser: &mut crate::esp::record::RecordParser<'_, 'b>,
    ) -> Result<Option<Self>, crate::esp::record::RecordParseError<'b>> {
        let data = match parser.try_parse::<SLSD>(SLSD)? {
            Some(value) => value,
            None => return Ok(None),
        };
        let name = parser.parse::<String>(SCVR)?;
        Ok(Some(Self { data, name }))
    }
}

#[derive(Debug)]
pub struct SLSD {
    pub index: u32,
    pub flags: SLSDFlags,
}

impl FromRecordBytes for SLSD {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((le_u32, take(12usize), SLSDFlags::parse, take(7usize))),
            |(index, _, flags, _)| Self { index, flags },
        )(input)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct SLSDFlags: u8 {
        const ENABLED = 0x0001;
    }
}

impl FromRecordBytes for SLSDFlags {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}

#[derive(Debug)]
pub struct SCHR {
    pub ref_count: u32,
    pub compiled_size: u32,
    pub variable_count: u32,
    pub ty: SCHRType,
    pub flags: SCHRFlags,
}

impl FromRecordBytes for SCHR {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((
                le_u32,
                le_u32,
                le_u32,
                enum_value::<SCHRType>,
                SCHRFlags::parse,
            )),
            |(ref_count, compiled_size, variable_count, ty, flags)| Self {
                ref_count,
                compiled_size,
                variable_count,
                ty,
                flags,
            },
        )(input)
    }
}

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u16)]
pub enum SCHRType {
    Object = 0x0,
    Quest = 0x1,
    Effect = 0x100,
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct SCHRFlags: u16 {
        const ENABLED = 0x0001;
    }
}

impl FromRecordBytes for SCHRFlags {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(le_u16, Self::from_bits_retain)(input)
    }
}
