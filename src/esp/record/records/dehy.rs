use super::{prelude::*, spel::SPEL};

/// Dehydration Stage
#[derive(Debug)]
pub struct DEHY {
    pub editor_id: EditorId,
    pub data: DehydrationStageData,
}

impl Record for DEHY {
    const TYPE: RecordType = DEHY;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let data: DehydrationStageData = parser.parse(DATA)?;
        Ok(Self { editor_id, data })
    }
}

#[derive(Debug)]
pub struct DehydrationStageData {
    pub trigger_threshold: u32,
    pub actor_effect: TypedFormId<SPEL>,
}

impl FromRecordBytes for DehydrationStageData {
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
