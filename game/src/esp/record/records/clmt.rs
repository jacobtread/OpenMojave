use super::{
    glob::GLOB,
    prelude::{model::ModelData, *},
    wthr::WTHR,
};

/// Climate
#[derive(Debug)]
pub struct CLMT {
    pub editor_id: EditorId,
    pub weather_types: Option<WeatherTypes>,
    pub sun_texture: Option<String>,
    pub sun_glare_texture: Option<String>,
    pub model_data: Option<ModelData>,
    pub timing: ClimateTiming,
}

impl Record for CLMT {
    const TYPE: RecordType = CLMT;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let weather_types: Option<WeatherTypes> = parser.try_parse(WLST)?;
        let sun_texture: Option<String> = parser.try_parse(FNAM)?;
        let sun_glare_texture: Option<String> = parser.try_parse(GNAM)?;
        let model_data: Option<ModelData> = ModelData::parse_first(parser)?;
        let timing: ClimateTiming = parser.parse(TNAM)?;

        Ok(Self {
            editor_id,
            weather_types,
            sun_texture,
            sun_glare_texture,
            model_data,
            timing,
        })
    }
}

#[derive(Debug)]
pub struct WeatherTypes {
    pub weather: NTypedFormId<WTHR>,
    pub chance: i32,
    pub global: NTypedFormId<GLOB>,
}

#[derive(Debug)]
pub struct ClimateTiming {
    pub sunrise_begin: u8,
    pub sunrise_end: u8,
    pub sunset_begin: u8,
    pub sunset_end: u8,
    pub volatility: u8,
    pub moon_phase_length: u8,
}

impl FromRecordBytes for WeatherTypes {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((NTypedFormId::parse, le_i32, NTypedFormId::parse)),
            |(weather, chance, global)| Self {
                weather,
                chance,
                global,
            },
        )(input)
    }
}
impl FromRecordBytes for ClimateTiming {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((u8, u8, u8, u8, u8, u8)),
            |(
                sunrise_begin,
                sunrise_end,
                sunset_begin,
                sunset_end,
                volatility,
                moon_phase_length,
            )| Self {
                sunrise_begin,
                sunrise_end,
                sunset_begin,
                sunset_end,
                volatility,
                moon_phase_length,
            },
        )(input)
    }
}
