use super::{prelude::*, spel::SPEL};

/// Sleep Deprivation Stage
#[derive(Debug)]
pub struct SLPD {
    pub editor_id: EditorId,
    pub data: SleepDeprivationData,
}

impl Record for SLPD {
    const TYPE: RecordType = RecordType::new(b"SLPD");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let data: SleepDeprivationData = parser.parse(DATA)?;
        Ok(Self { editor_id, data })
    }
}

#[derive(Debug)]
pub struct SleepDeprivationData {
    pub trigger_threshold: u32,
    pub actor_effect: TypedFormId<SPEL>,
}

impl FromRecordBytes for SleepDeprivationData {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((le_u32, TypedFormId::parse)),
            |(trigger_threshold, actor_effect)| Self {
                trigger_threshold,
                actor_effect,
            },
        )(input)
    }
}
