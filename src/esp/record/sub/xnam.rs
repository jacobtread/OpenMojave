use nom::{combinator::map, number::complete::le_i32, sequence::tuple};
use num_enum::TryFromPrimitive;

use crate::esp::{
    record::{enum_value, FromRecordBytes},
    shared::FormId,
};

#[derive(Debug)]
pub struct XNAM {
    /// FormID of the FACT or RACE record
    pub faction: FormId,
    pub modifier: i32,
    pub group_combat_reaction: GroupCombatReaction,
}

#[derive(Debug, Clone, TryFromPrimitive)]
#[repr(u32)]
pub enum GroupCombatReaction {
    Neutral = 0,
    Enemy = 1,
    Ally = 2,
    Friend = 3,
}

impl FromRecordBytes for XNAM {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(
            tuple((FormId::parse, le_i32, enum_value::<GroupCombatReaction>)),
            |(faction, modifier, group_combat_reaction)| Self {
                faction,
                modifier,
                group_combat_reaction,
            },
        )(input)
    }
}
