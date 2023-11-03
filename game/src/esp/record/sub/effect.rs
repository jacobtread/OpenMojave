use nom::{combinator::map, number::complete::le_u32, sequence::tuple};
use num_enum::TryFromPrimitive;

use crate::esp::{
    record::{enum_value, FromRecordBytes, RecordCollection},
    shared::TypedFormId,
};

use super::{actor_values::ActorValue, condition::CTDA, CTDA, EDID, EFIT};

#[derive(Debug)]
pub struct Effect {
    pub base_effect: Option<TypedFormId<() /* MGEF */>>,
    pub data: EFIT,
    pub condition: Option<CTDA>,
}

impl RecordCollection for Effect {
    fn parse_next<'b>(
        parser: &mut crate::esp::record::RecordParser<'_, 'b>,
    ) -> Result<Option<Self>, crate::esp::record::RecordParseError<'b>> {
        let base_effect = parser.try_parse::<TypedFormId<()>>(EDID)?;
        let data = match parser.try_parse::<EFIT>(EFIT)? {
            Some(value) => value,
            // TODO: If base_effect was present then error?
            None => return Ok(None),
        };
        let condition = parser.try_parse::<CTDA>(CTDA)?;
        Ok(Some(Self {
            base_effect,
            data,
            condition,
        }))
    }
}

#[derive(Debug)]
pub struct EFIT {
    pub magnitude: u32,
    pub area: u32,
    pub duration: u32,
    pub ty: EffectType,
    pub actor_value: ActorValue,
}

impl FromRecordBytes for EFIT {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(
            tuple((
                le_u32,
                le_u32,
                le_u32,
                enum_value::<EffectType>,
                enum_value::<ActorValue>,
            )),
            |(magnitude, area, duration, ty, actor_value)| Self {
                magnitude,
                area,
                duration,
                ty,
                actor_value,
            },
        )(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum EffectType {
    TSelf = 0,
    Touch = 1,
    Target = 2,
}
