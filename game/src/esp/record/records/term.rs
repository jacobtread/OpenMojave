use bitflags::bitflags;
use nom::{combinator::map, number::complete::u8, sequence::tuple, IResult};
use num_enum::TryFromPrimitive;

use crate::esp::{
    record::{
        enum_value,
        sub::{
            condition::CTDA, destruction::DestructionData, model::ModelData,
            object_bounds::ObjectBounds, script::Script, ANAM, CTDA, DESC, DNAM, EDID, FULL, INAM,
            ITXT, OBND, RNAM, SCRI, SNAM,
        },
        FromRecordBytes, Record, RecordCollection, RecordParseError, RecordParser, RecordType,
    },
    shared::{EditorId, TypedFormId},
};

use super::{scpt::SCPT, soun::SOUN};

#[derive(Debug)]
pub struct TERM {
    pub editor_id: EditorId,
    pub object_bounds: ObjectBounds,
    pub name: Option<String>,
    pub model_data: Option<ModelData>,
    pub script: Option<TypedFormId<SCPT>>,
    pub destruction_data: Option<DestructionData>,
    pub description: String,
    pub sound_looping: Option<TypedFormId<SOUN>>,
    pub password_note: Option<TypedFormId<() /* NOTE */>>,
    pub dnam: DNAM,
    pub menu_items: Vec<MenuItem>,
}

impl Record for TERM {
    const TYPE: RecordType = RecordType::new(b"TERM");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let object_bounds: ObjectBounds = parser.parse(OBND)?;
        let name: Option<String> = parser.try_parse(FULL)?;
        let model_data: Option<ModelData> = ModelData::parse_first(parser)?;
        let script: Option<TypedFormId<SCPT>> = parser.try_parse(SCRI)?;
        let destruction_data = DestructionData::parse_next(parser)?;
        let description: String = parser.parse(DESC)?;
        let sound_looping: Option<TypedFormId<SOUN>> = parser.try_parse(SNAM)?;
        let password_note: Option<TypedFormId<()>> = parser.try_parse(SNAM)?;
        let dnam: DNAM = parser.parse(DNAM)?;
        let menu_items: Vec<MenuItem> = parser.parse_collection()?;
        Ok(Self {
            editor_id,
            object_bounds,
            name,
            model_data,
            script,
            destruction_data,
            description,
            sound_looping,
            password_note,
            dnam,
            menu_items,
        })
    }
}

#[derive(Debug)]
pub struct DNAM {
    pub base_hacking_difficulty: BaseHackingDifficulty,
    pub flags: DNAMFlags,
    pub server_type: ServerType,
}

impl FromRecordBytes for DNAM {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((
                enum_value::<BaseHackingDifficulty>,
                DNAMFlags::parse,
                enum_value::<ServerType>,
                u8,
            )),
            |(base_hacking_difficulty, flags, server_type, _)| Self {
                base_hacking_difficulty,
                flags,
                server_type,
            },
        )(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum BaseHackingDifficulty {
    VeryEasy = 0,
    Easy = 1,
    Average = 2,
    Hard = 3,
    VeryHard = 4,
    RequiresKey = 5,
}

bitflags! {
    #[derive(Debug, Clone)]
    pub struct DNAMFlags: u8 {
        const LEVELED = 0x01;
        const UNLOCKED = 0x02;
        const ALTERNATE_COLORS = 0x04;
        const HIDE_WELCOME_TEXT_WHEN_DISPLAYING_TEXT =0x08;
    }
}

impl FromRecordBytes for DNAMFlags {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum ServerType {
    Server1 = 0,
    Server2 = 1,
    Server3 = 2,
    Server4 = 3,
    Server5 = 4,
    Server6 = 5,
    Server7 = 6,
    Server8 = 7,
    Server9 = 8,
    Server10 = 9,
}

#[derive(Debug)]
pub struct MenuItem {
    pub item_text: Option<String>,
    pub result_text: String,
    pub flags: ANAMFlags,
    pub display_note: Option<TypedFormId<() /* NOTE */>>,
    pub sub_menu: Option<TypedFormId<TERM>>,
    pub embedded_script: Script,
    pub conditions: Vec<CTDA>,
}

impl RecordCollection for MenuItem {
    fn parse_next<'b>(
        parser: &mut RecordParser<'_, 'b>,
    ) -> Result<Option<Self>, crate::esp::record::RecordParseError<'b>> {
        let item_text: Option<String> = parser.try_parse(ITXT)?;
        let result_text: String = match parser.try_parse(RNAM)? {
            Some(value) => value,
            None => return Ok(None),
        };
        let flags: ANAMFlags = parser.parse(ANAM)?;
        let display_note: Option<TypedFormId<()>> = parser.try_parse(INAM)?;
        let sub_menu: Option<TypedFormId<TERM>> = parser.try_parse(INAM)?;
        let embedded_script = Script::parse_next(parser)?
            .ok_or_else(|| RecordParseError::Custom("Missing menu item script".to_string()))?;
        let conditions: Vec<CTDA> = parser.try_parse_many(CTDA)?;
        Ok(Some(Self {
            item_text,
            result_text,
            flags,
            display_note,
            sub_menu,
            embedded_script,
            conditions,
        }))
    }
}

bitflags! {
    #[derive(Debug, Clone)]
    pub struct ANAMFlags: u8 {
        const LEVELED = 0x01;
        const UNLOCKED = 0x02;
        const ALTERNATE_COLORS = 0x04;
        const HIDE_WELCOME_TEXT_WHEN_DISPLAYING_TEXT =0x08;
    }
}

impl FromRecordBytes for ANAMFlags {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}
