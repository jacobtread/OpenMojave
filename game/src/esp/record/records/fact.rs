use super::prelude::*;
use crate::esp::record::sub::xnam::XNAM;

/// Faction
#[derive(Debug)]
pub struct FACT {
    pub editor_id: EditorId,
    pub name: Option<String>,
    pub relations: Vec<XNAM>,
    pub data: Option<FACTDATA>,
    pub ranks: Vec<FactionRank>,
    pub reputation: Option<FormId>,
}

impl Record for FACT {
    const TYPE: RecordType = RecordType::new(b"FACT");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let name: Option<String> = parser.try_parse(FULL)?;
        let relations: Vec<XNAM> = parser.try_parse_many(XNAM)?;
        let data: Option<FACTDATA> = parser.try_parse(DATA)?;

        parser.skip_type(CNAM); // Unused

        let ranks: Vec<FactionRank> = parser.parse_collection()?;
        let reputation: Option<FormId> = parser.try_parse(WMI1)?;

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
    // TODO: Can these be combined?
    pub flags_1: FACTFlags1,
    pub flags_2: FACTFlags2,
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct FACTFlags1: u8 {
        const HIDDEN_FROM_PC = 0x01;
        const EVIL           = 0x02;
        const SPECIAL_COMBAT = 0x4;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct FACTFlags2: u8 {
        const TRACK_CRIME = 0x01;
        const ALLOW_SELL  = 0x02;
    }
}

#[derive(Debug)]
pub struct FactionRank {
    pub rank_number: i32,
    pub male_name: String,
    pub female_name: String,
}

impl RecordCollection for FactionRank {
    fn parse_next<'b>(
        parser: &mut RecordParser<'_, 'b>,
    ) -> Result<Option<Self>, RecordParseError<'b>> {
        let rank_number: i32 = match parser.try_parse(RNAM)? {
            Some(value) => value,
            None => return Ok(None),
        };

        let male_name = parser.parse::<String>(MNAM)?;
        let female_name = parser.parse::<String>(FNAM)?;
        parser.skip_type(INAM);

        Ok(Some(FactionRank {
            rank_number,
            male_name,
            female_name,
        }))
    }
}

impl FromRecordBytes for FACTDATA {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((FACTFlags1::parse, FACTFlags2::parse, take(2usize))),
            |(flags_1, flags_2, _)| Self { flags_1, flags_2 },
        )(input)
    }
}

impl FromRecordBytes for FACTFlags1 {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}

impl FromRecordBytes for FACTFlags2 {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}
