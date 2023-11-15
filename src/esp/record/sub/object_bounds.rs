use crate::esp::record::FromRecordBytes;
use nalgebra::Vector3;
use nom::{combinator::map, sequence::tuple};

#[derive(Debug, Clone)]
pub struct ObjectBounds {
    pub start: Vector3<i16>,
    pub end: Vector3<i16>,
}

impl FromRecordBytes for ObjectBounds {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(tuple((Vector3::parse, Vector3::parse)), |(start, end)| {
            Self { start, end }
        })(input)
    }
}
