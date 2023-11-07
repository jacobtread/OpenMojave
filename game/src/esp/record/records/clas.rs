use super::prelude::*;
use crate::esp::record::sub::skill::Skill;

/// Class
#[derive(Debug)]
pub struct CLAS {
    pub editor_id: EditorId,
    pub name: String,
    pub description: String,
    pub large_icon_file_name: Option<String>,
    pub small_icon_file_name: Option<String>,
    pub data: CLASDATA,
    pub attributes: CLASATTR,
}

impl Record for CLAS {
    const TYPE: RecordType = CLAS;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let name: String = parser.parse(FULL)?;
        let description: String = parser.parse(DESC)?;

        let large_icon_file_name: Option<String> = parser.try_parse(ICON)?;
        let small_icon_file_name: Option<String> = parser.try_parse(MICO)?;

        let data: CLASDATA = parser.parse(DATA)?;
        let attributes: CLASATTR = parser.parse(ATTR)?;

        Ok(Self {
            editor_id,
            name,
            description,
            large_icon_file_name,
            small_icon_file_name,
            data,
            attributes,
        })
    }
}

#[derive(Debug)]
pub struct CLASDATA {
    pub tag_skill_1: i32,
    pub tag_skill_2: i32,
    pub tag_skill_3: i32,
    pub tag_skill_4: i32,
    pub flags: CLASDataFlags,
    pub buy_sell_services: ServiceFlags,
    pub teaches: Skill,
    pub maximum_training_level: u8,
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct CLASDataFlags: u32 {
        const PLAYABLE = 0x00000001;
        const GUARD = 0x00000002;
    }
}

#[derive(Debug)]
pub struct CLASATTR {
    pub strength: u8,
    pub perception: u8,
    pub endurance: u8,
    pub charisma: u8,
    pub intelligence: u8,
    pub agility: u8,
    pub luck: u8,
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct ServiceFlags: u32 {
        const WEAPONS   = 0x00000001;
        const ARMOR     = 0x00000002;
        const ALCHOL    = 0x00000004;
        const BOOKS     = 0x00000008;
        const FOOD      = 0x00000010;
        const CHEMS     = 0x00000020;
        const STIMPACKS = 0x00000040;
        const LIGHTS    = 0x00000080;
        const U1        = 0x00000100;
        const U2        = 0x00000200;
        const MISC      = 0x00000400;
        const U3        = 0x00000800;
        const U4        = 0x00001000;
        const POTIONS   = 0x00002000;
        const TRAINING  = 0x00004000;
        const U5        = 0x00008000;
        const RECHARGE  = 0x00010000;
        const REPAIR    = 0x00020000;
    }
}

impl FromRecordBytes for CLASDataFlags {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(le_u32, Self::from_bits_retain)(input)
    }
}

impl FromRecordBytes for CLASDATA {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((
                le_i32,
                le_i32,
                le_i32,
                le_i32,
                CLASDataFlags::parse,
                ServiceFlags::parse,
                enum_value::<Skill>,
                u8,
                // Unused
                take(2usize),
            )),
            |(
                tag_skill_1,
                tag_skill_2,
                tag_skill_3,
                tag_skill_4,
                flags,
                buy_sell_services,
                teaches,
                maximum_training_level,
                _,
            )| Self {
                tag_skill_1,
                tag_skill_2,
                tag_skill_3,
                tag_skill_4,
                flags,
                buy_sell_services,
                teaches,
                maximum_training_level,
            },
        )(input)
    }
}

impl FromRecordBytes for CLASATTR {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((u8, u8, u8, u8, u8, u8, u8)),
            |(strength, perception, endurance, charisma, intelligence, agility, luck)| Self {
                strength,
                perception,
                endurance,
                charisma,
                intelligence,
                agility,
                luck,
            },
        )(input)
    }
}

impl FromRecordBytes for ServiceFlags {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(le_u32, Self::from_bits_retain)(input)
    }
}
