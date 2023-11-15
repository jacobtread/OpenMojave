use super::{prelude::*, spel::SPEL};

/// Hunger Stage
#[derive(Debug)]
pub struct HUNG {
    pub editor_id: EditorId,
    pub data: HungerStageData,
}

impl Record for HUNG {
    const TYPE: RecordType = RecordType::new(b"HUNG");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let data: HungerStageData = parser.parse(DATA)?;

        Ok(Self { editor_id, data })
    }
}

#[derive(Debug)]
pub struct HungerStageData {
    pub trigger_threshold: u32,
    pub actor_effect: TypedFormId<SPEL>,
}

impl FromRecordBytes for HungerStageData {
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
