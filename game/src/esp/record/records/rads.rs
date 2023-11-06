use super::{prelude::*, spel::SPEL};

/// Radiation Stage
#[derive(Debug)]
pub struct RADS {
    pub editor_id: EditorId,
    pub data: RadiationStageData,
}

impl Record for RADS {
    const TYPE: RecordType = RecordType::new(b"RADS");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let data: RadiationStageData = parser.parse(DATA)?;
        Ok(Self { editor_id, data })
    }
}

#[derive(Debug)]
pub struct RadiationStageData {
    pub trigger_threshold: u32,
    pub actor_effect: TypedFormId<SPEL>,
}

impl FromRecordBytes for RadiationStageData {
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
