use crate::esp::shared::EditorId;
use crate::esp::{records::record::RecordType, shared::FormId};
use nom::combinator::{map, rest};
use nom::number::complete::le_f32;
use nom::IResult;

use super::parse_cstring;

pub struct NAME;

impl NAME {
    pub const TYPE: RecordType = RecordType::from_value(b"NAME");

    #[inline]
    pub fn parse(input: &[u8]) -> IResult<&[u8], FormId> {
        FormId::parse(input)
    }
}

pub struct EDID;

impl EDID {
    pub const TYPE: RecordType = RecordType::from_value(b"EDID");

    pub fn parse(input: &[u8]) -> IResult<&[u8], EditorId> {
        map(parse_cstring, EditorId)(input)
    }
}

pub struct CNAM;

impl CNAM {
    pub const TYPE: RecordType = RecordType::from_value(b"CNAM");

    #[inline]
    pub fn parse_string(input: &[u8]) -> IResult<&[u8], String> {
        parse_cstring(input)
    }

    #[inline]
    pub fn parse_float(input: &[u8]) -> IResult<&[u8], f32> {
        le_f32(input)
    }

    #[inline]
    pub fn parse_form_id(input: &[u8]) -> IResult<&[u8], FormId> {
        FormId::parse(input)
    }
}

pub struct SNAM;

impl SNAM {
    pub const TYPE: RecordType = RecordType::from_value(b"SNAM");

    #[inline]
    pub fn parse_string(input: &[u8]) -> IResult<&[u8], String> {
        parse_cstring(input)
    }

    #[inline]
    pub fn parse_float(input: &[u8]) -> IResult<&[u8], f32> {
        le_f32(input)
    }

    #[inline]
    pub fn parse_form_id(input: &[u8]) -> IResult<&[u8], FormId> {
        FormId::parse(input)
    }
}

pub struct ONAM;

impl ONAM {
    pub const TYPE: RecordType = RecordType::from_value(b"ONAM");

    #[inline]
    pub fn parse_string(input: &[u8]) -> IResult<&[u8], String> {
        parse_cstring(input)
    }

    #[inline]
    pub fn parse_float(input: &[u8]) -> IResult<&[u8], f32> {
        le_f32(input)
    }

    #[inline]
    pub fn parse_form_id(input: &[u8]) -> IResult<&[u8], FormId> {
        FormId::parse(input)
    }
}

pub struct DATA;

impl DATA {
    pub const TYPE: RecordType = RecordType::from_value(b"DATA");

    pub fn parse_bytes(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
        map(rest, |rest: &[u8]| rest.to_vec())(input)
    }

    pub fn parse_skipping(input: &[u8]) -> IResult<&[u8], ()> {
        map(rest, |_| ())(input)
    }
}

/// Encounter zone, FormID of an ECZN record
pub struct XEZN;

impl XEZN {
    pub const TYPE: RecordType = RecordType::from_value(b"XEZN");

    #[inline]
    pub fn parse(input: &[u8]) -> IResult<&[u8], FormId> {
        FormId::parse(input)
    }
}
