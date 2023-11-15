use nom::{bytes::complete::take, combinator::map, number::complete::le_u32, sequence::tuple};
use num_enum::TryFromPrimitive;

use crate::esp::{
    record::{enum_value, take4, FromRecordBytes},
    shared::FormId,
};

/// CTDA
#[derive(Debug)]
pub struct CTDA {
    pub ty: ConditionType,
    pub comparison_value: RawComparisonValue,
    pub func_index: u32,
    pub param_1: [u8; 4],
    pub param_2: [u8; 4],
    pub run_on: RunOn,
    // A FormID of a PLYR, ACHR, ACRE, REFR, PMIS or PGRE reference on which to apply the function, or null.
    pub reference: FormId,
}

impl FromRecordBytes for CTDA {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(
            tuple((
                enum_value::<ConditionType>,
                // This might be take 3 instead (Docs seem incorrect, changed to 2 because ty is u16 not u8 like doc suggests)
                take(2usize),
                RawComparisonValue::parse,
                le_u32,
                take4,
                take4,
                enum_value::<RunOn>,
                FormId::parse,
            )),
            |(ty, _, comparison_value, func_index, param_1, param_2, run_on, reference)| Self {
                ty,
                comparison_value,
                func_index,
                param_1,
                param_2,
                run_on,
                reference,
            },
        )(input)
    }
}

/// 4 bytes which must either be a FormId for a GLOB record or a
/// f32 value
#[derive(Debug)]
pub struct RawComparisonValue([u8; 4]);

impl FromRecordBytes for RawComparisonValue {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(take4, Self)(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u16)]
pub enum ConditionType {
    // Combine next condition using OR (default is to use AND)
    CombineOr = 0x0001,
    RunOnTarget = 0x0002,
    UseGlobal = 0x0004,
    EqualTo = 0x0000,
    NotEqualTo = 0x2000,
    GreaterThan = 0x4000,
    GreaterThanOrEqual = 0x6000,
    LessThan = 0x8000,
    LessThanOrEqual = 0xA000,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum RunOn {
    Subject = 0,
    Target = 1,
    Reference = 2,
    CombatTarget = 3,
    LinkedReference = 4,
}
