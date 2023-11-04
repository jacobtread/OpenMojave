use bitflags::bitflags;
use nom::{
    bytes::complete::take,
    combinator::map,
    number::complete::{le_u32, u8},
    sequence::tuple,
    IResult,
};
use num_enum::TryFromPrimitive;

use crate::esp::{
    record::{
        enum_value,
        sub::{effect::Effect, EDID, FULL, SPIT},
        FromRecordBytes, Record, RecordParseError, RecordParser, RecordType,
    },
    shared::EditorId,
};

/// Actor Effect / Spell
#[derive(Debug)]
pub struct SPEL {
    pub editor_id: EditorId,
    pub name: Option<String>,
    pub spit: SPIT,
    pub effects: Vec<Effect>,
}

impl Record for SPEL {
    const TYPE: RecordType = RecordType::new(b"SPEL");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id = parser.parse::<EditorId>(EDID)?;
        let name = parser.try_parse::<String>(FULL)?;
        let spit = parser.parse::<SPIT>(SPIT)?;
        let effects = parser.parse_collection::<Effect>()?;
        if effects.is_empty() {
            return Err(crate::esp::record::RecordParseError::Custom(
                "Missing SPEL effect".to_string(),
            ));
        }

        Ok(Self {
            editor_id,
            name,
            spit,
            effects,
        })
    }
}

#[derive(Debug)]
pub struct SPIT {
    pub ty: SPITType,
    pub flags: SPITFlags,
}

impl FromRecordBytes for SPIT {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((
                enum_value::<SPITType>,
                le_u32,
                le_u32,
                SPITFlags::parse,
                take(3usize),
            )),
            |(ty, _cost, _level, flags, _unused)| Self { ty, flags },
        )(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum SPITType {
    ActorEffect = 0,
    Disease = 1,
    Power = 2,
    LesserPower = 3,
    Ability = 4,
    Poison = 5,
    U6 = 6,
    U7 = 7,
    U8 = 8,
    U9 = 9,
    Addiction = 10,
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct SPITFlags: u8 {
        const NO_AUTO_CALC = 0x01;
        const IMMUNE_TO_SILENCE_1 = 0x02;
        const PC_START_EFFECT = 0x04;
        const IMMUNE_TO_SILENCE_2 = 0x08;
        const AREA_EFFECT_IGNORES_LOS = 0x10;
        const SCRIPT_EFFECT_ALWAYS_APPLIES = 0x20;
        const DISABLE_ABSORB = 0x40;
        const FORCE_TOUCH_EXPLODE = 0x80;
    }
}

impl FromRecordBytes for SPITFlags {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}
