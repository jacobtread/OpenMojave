use crate::esp::records::record::{RecordType, SubRecord};
use bitflags::bitflags;
use nom::{
    bytes::complete::take,
    character::complete::u8,
    combinator::{map, map_res},
    number::complete::{le_u16, le_u32},
    sequence::tuple,
    IResult,
};

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct SLSDFlags: u8 {
        const IS_LONG_OR_SHORT = 0x01;
    }
}

impl SLSDFlags {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}

/// Basic Script Data
#[derive(Debug)]
pub struct SLSD {
    pub index: u32,
    pub flags: SLSDFlags,
}

impl SubRecord for SLSD {
    const TYPE: RecordType = RecordType::from_value(b"SLSD");

    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(
            tuple((
                le_u32,
                // Unused
                take(12usize),
                SLSDFlags::parse,
                // Unused
                take(7usize),
            )),
            |(index, _, flags, _)| Self { index, flags },
        )(input)
    }
}
