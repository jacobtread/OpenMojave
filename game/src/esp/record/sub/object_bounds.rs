use nom::{combinator::map, number::complete::le_i16, sequence::tuple};

use crate::esp::record::FromRecordBytes;

#[derive(Debug, Clone)]
pub struct ObjectBounds {
    pub x1: i16,
    pub y1: i16,
    pub z1: i16,

    pub x2: i16,
    pub y2: i16,
    pub z2: i16,
}

impl FromRecordBytes for ObjectBounds {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(
            tuple((le_i16, le_i16, le_i16, le_i16, le_i16, le_i16)),
            |(x1, y1, z1, x2, y2, z2)| Self {
                x1,
                y1,
                z1,
                x2,
                y2,
                z2,
            },
        )(input)
    }
}
