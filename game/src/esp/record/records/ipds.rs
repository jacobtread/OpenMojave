use super::{ipct::IPCT, prelude::*};

/// Impact Dataset
#[derive(Debug)]
pub struct IPDS {
    pub editor_id: EditorId,
    pub impacts: Impacts,
}

impl Record for IPDS {
    const TYPE: RecordType = RecordType::new(b"IPDS");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let impacts: Impacts = parser.parse(DATA)?;
        Ok(Self { editor_id, impacts })
    }
}

#[derive(Debug)]
pub struct Impacts {
    pub stone: NTypedFormId<IPCT>,
    pub dirt: NTypedFormId<IPCT>,
    pub grass: NTypedFormId<IPCT>,
    pub glass: NTypedFormId<IPCT>,
    pub metal: NTypedFormId<IPCT>,
    pub wood: NTypedFormId<IPCT>,
    pub organic: NTypedFormId<IPCT>,
    pub cloth: NTypedFormId<IPCT>,
    pub water: NTypedFormId<IPCT>,
    pub hollow_metal: NTypedFormId<IPCT>,
    pub organic_bug: NTypedFormId<IPCT>,
    pub organic_glow: NTypedFormId<IPCT>,
}

impl FromRecordBytes for Impacts {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((
                NTypedFormId::parse,
                NTypedFormId::parse,
                NTypedFormId::parse,
                NTypedFormId::parse,
                NTypedFormId::parse,
                NTypedFormId::parse,
                NTypedFormId::parse,
                NTypedFormId::parse,
                NTypedFormId::parse,
                NTypedFormId::parse,
                NTypedFormId::parse,
                NTypedFormId::parse,
            )),
            |(
                stone,
                dirt,
                grass,
                glass,
                metal,
                wood,
                organic,
                cloth,
                water,
                hollow_metal,
                organic_bug,
                organic_glow,
            )| Self {
                stone,
                dirt,
                grass,
                glass,
                metal,
                wood,
                organic,
                cloth,
                water,
                hollow_metal,
                organic_bug,
                organic_glow,
            },
        )(input)
    }
}
