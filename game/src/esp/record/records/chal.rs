use crate::esp::record::take_bytes_const;

use super::{prelude::*, scpt::SCPT};

/// Challenge
#[derive(Debug)]
pub struct CHAL {
    pub editor_id: EditorId,
    pub name: Option<String>,
    /// Path to icon texture, when viewed in PipBoy
    pub pip_icon: Option<String>,
    ///Path to icon texture, when viewed in upper-left message
    pub upper_left_icon: Option<String>,
    pub script: Option<TypedFormId<SCPT>>,
    pub description: Option<String>,
    pub data: Option<ChallengeData>,
    pub value_3: Option<FormId>,
    pub value_4: Option<FormId>,
}

impl Record for CHAL {
    const TYPE: RecordType = CHAL;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let name: Option<String> = parser.try_parse(FULL)?;
        let pip_icon: Option<String> = parser.try_parse(ICON)?;
        let upper_left_icon: Option<String> = parser.try_parse(MICO)?;
        let script: Option<TypedFormId<SCPT>> = parser.try_parse(SCRI)?;
        let description: Option<String> = parser.try_parse(DESC)?;
        let data: Option<ChallengeData> = parser.try_parse(DATA)?;
        let value_3: Option<FormId> = parser.try_parse(SNAM)?;
        let value_4: Option<FormId> = parser.try_parse(XNAM)?;
        Ok(Self {
            editor_id,
            name,
            pip_icon,
            upper_left_icon,
            script,
            description,
            data,
            value_3,
            value_4,
        })
    }
}

#[derive(Debug)]
pub struct ChallengeData {
    pub ty: ChallengeType,
    pub threshold: u32,
    pub flags: ChallengeFlags,
    pub interval: u32,
    pub value_1: [u8; 2],
    pub value_2: [u8; 2],
    pub value_3: [u8; 4],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum ChallengeType {
    KillFromAFormList = 0,
    KillASpecificFormId = 1,
    KillAnyInACategory = 2,
    HitAnEnemy = 3,
    DiscoverAMapMarker = 4,
    UseAnItem = 5,
    AquireAnItem = 6,
    UseASkill = 7,
    DoDamage = 8,
    UseAnItemFromAList = 9,
    AquireAnItemFromAList = 10,
    MiscellaneousStat = 11,
    CraftUsingAnItem = 12,
    ScriptedChallenge = 13,
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct ChallengeFlags: u32 {
        const START_DISABLED     = 0x00000001;
        const RECURRING          = 0x00000002;
        const SHOW_ZERO_PROGRESS = 0x00000004;
    }
}

impl FromRecordBytes for ChallengeData {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(
            tuple((
                enum_value,
                le_u32,
                ChallengeFlags::parse,
                le_u32,
                take_bytes_const,
                take_bytes_const,
                take_bytes_const,
            )),
            |(ty, threshold, flags, interval, value_1, value_2, value_3)| Self {
                ty,
                threshold,
                flags,
                interval,
                value_1,
                value_2,
                value_3,
            },
        )(input)
    }
}

impl FromRecordBytes for ChallengeFlags {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(le_u32, Self::from_bits_retain)(input)
    }
}
