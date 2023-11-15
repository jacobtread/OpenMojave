use super::{
    imad::IMAD,
    prelude::{model::ModelData, *},
};

/// Camera Shot
#[derive(Debug)]
pub struct CAMS {
    pub editor_id: EditorId,
    pub model_data: Option<ModelData>,
    pub data: CameraShotData,
    pub image_space_modifier: Option<TypedFormId<IMAD>>,
}

impl Record for CAMS {
    const TYPE: RecordType = CAMS;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let model_data: Option<ModelData> = ModelData::parse_first(parser)?;
        let data: CameraShotData = parser.parse(DATA)?;
        let image_space_modifier: Option<TypedFormId<IMAD>> = parser.try_parse(MNAM)?;

        Ok(Self {
            editor_id,
            model_data,
            data,
            image_space_modifier,
        })
    }
}

#[derive(Debug)]
pub struct CameraShotData {
    pub action: Action,
    pub location: LocationTarget,
    pub target: LocationTarget,
    pub flags: CameraShotFlags,
    pub player_time_multiplier: f32,
    pub target_time_multiplier: f32,
    pub global_time_multiplier: f32,
    pub max_time: f32,
    pub min_time: f32,
    pub target_percent_between_actors: f32,
}

impl FromRecordBytes for CameraShotData {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((
                enum_value,
                enum_value,
                enum_value,
                CameraShotFlags::parse,
                le_f32,
                le_f32,
                le_f32,
                le_f32,
                le_f32,
                le_f32,
            )),
            |(
                action,
                location,
                target,
                flags,
                player_time_multiplier,
                target_time_multiplier,
                global_time_multiplier,
                max_time,
                min_time,
                target_percent_between_actors,
            )| Self {
                action,
                location,
                target,
                flags,
                player_time_multiplier,
                target_time_multiplier,
                global_time_multiplier,
                max_time,
                min_time,
                target_percent_between_actors,
            },
        )(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum Action {
    Shoot = 0,
    Fly = 1,
    Hit = 2,
    Zoom = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum LocationTarget {
    Attacker = 0,
    Projectile = 1,
    Target = 2,
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct CameraShotFlags: u32 {
        const POSITION_FOLLOWS_LOCATION = 0x00000001;
        const POSITION_FOLLOWS_TARGET   = 0x00000002;
        const DONT_FOLLOW_BONE          = 0x00000004;
        const FIRST_PERSON_CAMERA       = 0x00000008;
        const NO_TRACER                 = 0x00000010;
        const START_AT_TIME_ZERO         = 0x00000020;
    }
}
impl FromRecordBytes for CameraShotFlags {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(le_u32, Self::from_bits_retain)(input)
    }
}
