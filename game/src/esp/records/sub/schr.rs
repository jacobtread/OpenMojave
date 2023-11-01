use crate::esp::records::record::{RecordType, SubRecord};
use bitflags::bitflags;
use nom::{
    combinator::{map, map_res},
    number::complete::{le_u16, le_u32},
    sequence::tuple,
    IResult,
};
use num_enum::TryFromPrimitive;

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct SCHRFlags: u16 {
        const ENABLED = 0x0001;
    }
}

impl SCHRFlags {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(le_u16, Self::from_bits_retain)(input)
    }
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u16)]
pub enum SCHRType {
    Object = 0x0,
    Quest = 0x1,
    Effect = 0x100,
}

impl SCHRType {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map_res(le_u16, Self::try_from_primitive)(input)
    }
}

/// Basic Script Data
#[derive(Debug)]
pub struct SCHR {
    pub unused: u32,
    pub ref_count: u32,
    pub compiled_size: u32,
    pub variable_count: u32,
    pub ty: SCHRType,
    pub flags: SCHRFlags,
}

impl SubRecord for SCHR {
    const TYPE: RecordType = RecordType::from_value(b"SCHR");

    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(
            tuple((
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                SCHRType::parse,
                SCHRFlags::parse,
            )),
            |(unused, ref_count, compiled_size, variable_count, ty, flags)| Self {
                unused,
                ref_count,
                compiled_size,
                variable_count,
                ty,
                flags,
            },
        )(input)
    }
}
