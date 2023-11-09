use super::prelude::*;

/// Encounter Zone
#[derive(Debug)]
pub struct ECZN {
    pub editor_id: EditorId,
    pub data: EncounterData,
}

impl Record for ECZN {
    const TYPE: RecordType = ECZN;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let data: EncounterData = parser.parse(DATA)?;

        Ok(Self { editor_id, data })
    }
}

#[derive(Debug)]
pub struct EncounterData {
    /// FormID of an NPC_ or FACT record, or null.
    pub owner: FormId,
    pub rank: i8,
    pub min_level: i8,
    pub flags: EncounterFlags,
}

bitflags! {
    #[derive(Debug, Clone)]
    pub struct EncounterFlags: u8 {
        const NEVER_RESETS = 0x01;
        const MATCH_PC_BELOW_MIN_LEVEL = 0x02;

    }
}

impl FromRecordBytes for EncounterData {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((FormId::parse, i8, i8, EncounterFlags::parse, i8)),
            |(owner, rank, min_level, flags, _unused)| Self {
                owner,
                rank,
                min_level,
                flags,
            },
        )(input)
    }
}

impl FromRecordBytes for EncounterFlags {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}
