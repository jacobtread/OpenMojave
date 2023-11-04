use crate::esp::{
    record::{
        sub::{xnam::XNAM, CNAM, DATA, EDID, FNAM, FULL, INAM, MNAM, RNAM, WMI1, XNAM},
        FromRecordBytes, Record, RecordParseError, RecordParser, RecordType,
    },
    shared::{EditorId, FormId},
};
use bitflags::bitflags;
use nom::{bytes::complete::take, combinator::map, number::complete::u8, sequence::tuple, IResult};

#[derive(Debug)]
pub struct FACT {
    pub editor_id: EditorId,
    pub name: Option<String>,
    pub relations: Vec<XNAM>,
    pub data: Option<FACTDATA>,
    pub ranks: Vec<Rank>,
    pub reputation: Option<FormId>,
}

impl Record for FACT {
    const TYPE: RecordType = RecordType::new(b"FACT");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id = parser.parse::<EditorId>(EDID)?;
        let name = parser.try_parse::<String>(FULL)?;

        let mut relations: Vec<XNAM> = Vec::new();
        while let Some(relation) = parser.try_parse::<XNAM>(XNAM)? {
            relations.push(relation);
        }

        let data = parser.try_parse::<FACTDATA>(DATA)?;

        // Unused
        parser.skip_type(CNAM);

        let mut ranks: Vec<Rank> = Vec::new();

        while let Some(rank_number) = parser.try_parse::<i32>(RNAM)? {
            let male_name = parser.parse::<String>(MNAM)?;
            let female_name = parser.parse::<String>(FNAM)?;
            parser.skip_type(INAM);
            ranks.push(Rank {
                rank_number,
                male_name,
                female_name,
            });
        }

        let reputation = parser.try_parse::<FormId>(WMI1)?;

        Ok(Self {
            editor_id,
            name,
            relations,
            data,
            ranks,
            reputation,
        })
    }
}

#[derive(Debug)]
pub struct FACTDATA {
    pub flags_1: FACTFlags1,
    pub flags_2: FACTFlags2,
}

impl FromRecordBytes for FACTDATA {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((FACTFlags1::parse, FACTFlags2::parse, take(2usize))),
            |(flags_1, flags_2, _)| Self { flags_1, flags_2 },
        )(input)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct FACTFlags1: u8 {
        const HIDDEN_FROM_PC = 0x01;
        const EVIL           = 0x02;
        const SPECIAL_COMBAT = 0x4;
    }
}

impl FromRecordBytes for FACTFlags1 {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct FACTFlags2: u8 {
        const TRACK_CRIME = 0x01;
        const ALLOW_SELL  = 0x02;
    }
}

impl FromRecordBytes for FACTFlags2 {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}

#[derive(Debug)]
pub struct Rank {
    pub rank_number: i32,
    pub male_name: String,
    pub female_name: String,
}
