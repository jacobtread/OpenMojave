use super::{
    amef::AMEF,
    prelude::{destruction::DestructionData, model::ModelData, object_bounds::ObjectBounds, *},
    proj::PROJ,
    scpt::SCPT,
    soun::SOUN,
};

/// Ammunition
#[derive(Debug)]
pub struct AMMO {
    pub editor_id: EditorId,
    pub object_bounds: ObjectBounds,
    pub name: String,
    pub model_data: Option<ModelData>,
    pub large_icon_file_name: Option<String>,
    pub small_icon_file_name: Option<String>,
    pub script: Option<TypedFormId<SCPT>>,
    pub destruction_data: Option<DestructionData>,
    pub sound_pick_up: Option<TypedFormId<SOUN>>,
    pub sound_drop: Option<TypedFormId<SOUN>>,
    pub data: AmmoData,
    pub data_2: Option<AmmoData2>,
    pub short_name: Option<String>,
    pub abbreviation: Option<String>,
    pub effects: Vec<TypedFormId<AMEF>>,
}

impl Record for AMMO {
    const TYPE: RecordType = AMMO;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let object_bounds: ObjectBounds = parser.parse(OBND)?;
        let name: String = parser.parse(FULL)?;
        let model_data: Option<ModelData> = ModelData::parse_first(parser)?;
        let large_icon_file_name: Option<String> = parser.try_parse(ICON)?;
        let small_icon_file_name: Option<String> = parser.try_parse(MICO)?;
        let script: Option<TypedFormId<SCPT>> = parser.try_parse(SCRI)?;
        let destruction_data: Option<DestructionData> = DestructionData::parse_next(parser)?;
        let sound_pick_up: Option<TypedFormId<SOUN>> = parser.try_parse(YNAM)?;
        let sound_drop: Option<TypedFormId<SOUN>> = parser.try_parse(ZNAM)?;
        let data: AmmoData = parser.parse(DATA)?;
        let data_2: Option<AmmoData2> = parser.try_parse(DAT2)?;
        let short_name: Option<String> = parser.try_parse(ONAM)?;
        let abbreviation: Option<String> = parser.try_parse(QNAM)?;
        let effects: Vec<TypedFormId<AMEF>> = parser.try_parse_many(RCIL)?;

        Ok(Self {
            editor_id,
            object_bounds,
            name,
            model_data,
            large_icon_file_name,
            small_icon_file_name,
            script,
            destruction_data,
            sound_pick_up,
            sound_drop,
            data,
            data_2,
            short_name,
            abbreviation,
            effects,
        })
    }
}

#[derive(Debug)]
pub struct AmmoData {
    pub speed: f32,
    pub flags: AmmoFlags,
    pub value: i32,
    pub clip_rounds: u8,
}

#[derive(Debug)]
pub struct AmmoData2 {
    pub projectiles_per_shot: u32,
    pub projectile: NTypedFormId<PROJ>,
    pub weight: f32,
    // FormID of an AMMO or MISC record, or null.
    pub consumed_ammo: FormId,
    pub consumed_percentage: f32,
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct AmmoFlags: u8 {
        const IGNORES_NORMAL_WEAPON_RESISTANCE = 0x01;
        const NON_PLAYABLE                     = 0x02;
    }
}

impl FromRecordBytes for AmmoData {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((le_f32, AmmoFlags::parse, take(3usize), le_i32, u8)),
            |(speed, flags, _, value, clip_rounds)| Self {
                speed,
                flags,
                value,
                clip_rounds,
            },
        )(input)
    }
}
impl FromRecordBytes for AmmoData2 {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((le_u32, NTypedFormId::parse, le_f32, FormId::parse, le_f32)),
            |(projectiles_per_shot, projectile, weight, consumed_ammo, consumed_percentage)| Self {
                projectiles_per_shot,
                projectile,
                weight,
                consumed_ammo,
                consumed_percentage,
            },
        )(input)
    }
}

impl FromRecordBytes for AmmoFlags {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}
