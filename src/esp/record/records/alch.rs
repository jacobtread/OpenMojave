use super::{
    prelude::{equipment_type::EquipmentType, *},
    scpt::SCPT,
    soun::SOUN,
    spel::SPEL,
};
use crate::esp::record::sub::{
    destruction::DestructionData, effect::Effect, model::ModelData, object_bounds::ObjectBounds,
};

/// Ingestible
#[derive(Debug)]
pub struct ALCH {
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
    pub equipment_type: EquipmentType,
    pub weight: f32,
    pub data: IngestibleData,
    pub effects: Vec<Effect>,
}

impl Record for ALCH {
    const TYPE: RecordType = ALCH;

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
        let equipment_type: EquipmentType = parser.parse(ETYP)?;
        let weight: f32 = parser.parse(DATA)?;
        let effect_data: IngestibleData = parser.parse(ENIT)?;
        let effects = parser.parse_collection::<Effect>()?;
        if effects.is_empty() {
            return Err(RecordParseError::Custom("Missing ALCH effect".to_string()));
        }

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
            equipment_type,
            weight,
            data: effect_data,
            effects,
        })
    }
}

#[derive(Debug)]
pub struct IngestibleData {
    pub value: i32,
    pub flags: IngestibleFlags,
    pub withdrawal_effect: NTypedFormId<SPEL>,
    pub addiction_chance: f32,
    pub sound_consume: NTypedFormId<SOUN>,
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct IngestibleFlags: u8 {
        const NO_AUTO_CALCULATION = 0x01;
        const FOOD_ITEM           = 0x02;
        const MEDICINE            = 0x04;
    }
}

impl FromRecordBytes for IngestibleData {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(
            tuple((
                le_i32,
                IngestibleFlags::parse,
                take(3usize),
                NTypedFormId::parse,
                le_f32,
                NTypedFormId::parse,
            )),
            |(value, flags, _, withdrawal_effect, addiction_chance, sound_consume)| Self {
                value,
                flags,
                withdrawal_effect,
                addiction_chance,
                sound_consume,
            },
        )(input)
    }
}

impl FromRecordBytes for IngestibleFlags {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}
