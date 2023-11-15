use super::{
    debr::DEBR,
    expl::EXPL,
    ipds::IPDS,
    prelude::{actor_values::ActorValue, model::ModelData, *},
    rgdl::RGDL,
};

/// Body Part Data
#[derive(Debug)]
pub struct BPTD {
    pub editor_id: EditorId,
    pub model_data: ModelData,
    pub body_part: Vec<BodyPart>,
    pub unnamed_body_part: Vec<UnnamedBodyPart>,
    pub ragdoll: Option<TypedFormId<RGDL>>,
}

impl Record for BPTD {
    const TYPE: RecordType = BPTD;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let model_data: ModelData = ModelData::require(parser)?;
        let body_part: Vec<BodyPart> = parser.parse_collection()?;
        if body_part.is_empty() {
            return Err(RecordParseError::Custom("Missing body parts".to_string()));
        }
        let unnamed_body_part: Vec<UnnamedBodyPart> = parser.parse_collection()?;
        if unnamed_body_part.is_empty() {
            return Err(RecordParseError::Custom(
                "Missing unnamed body parts".to_string(),
            ));
        }
        let ragdoll: Option<TypedFormId<RGDL>> = parser.try_parse(RAGA)?;

        Ok(Self {
            editor_id,
            model_data,
            body_part,
            unnamed_body_part,
            ragdoll,
        })
    }
}

#[derive(Debug)]
pub struct BodyPart {
    pub part_name: String,
    pub part_node: String,
    pub vats_target: String,
    pub ik_data_start_node: String,
    pub bpnd: BPND,
    pub limb_replacement_mode: String,
    pub gore_effects_target_bone: String,
}
#[derive(Debug)]
pub struct UnnamedBodyPart {
    pub part_node: String,
    pub vats_target: String,
    pub ik_data_start_node: String,
    pub bpnd: BPND,
    pub limb_replacement_mode: String,
    pub gore_effects_target_bone: String,
}

#[derive(Debug)]
pub struct BPND {
    pub damage_multiplier: f32,
    pub flags: BodyPartFlag,
    pub part_type: PartType,
    pub health_percent: u8,
    pub actor_value: ActorValue,
    pub to_hit_chance: u8,
    pub ex_explosion_chance_percent: u8,
    pub ex_debris_count: u16,
    pub ex_debris: NTypedFormId<DEBR>,
    pub ex_explosion: NTypedFormId<EXPL>,
    pub tracking_max_angle: f32,
    pub ex_debris_scale: f32,

    pub sv_debris_count: u16,
    pub sv_debris: NTypedFormId<DEBR>,
    pub sv_explosion: NTypedFormId<EXPL>,
    pub sv_debris_scale: f32,

    pub gr_translate: Vector3<f32>,
    pub gr_rotation: Vector3<f32>,

    pub sv_impact_dataset: NTypedFormId<IPDS>,
    pub ex_impact_dataset: NTypedFormId<IPDS>,

    pub sv_decal_count: u8,
    pub ex_decal_count: u8,

    pub limb_replacement_scale: f32,
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct BodyPartFlag: u8 {
        const SEVERABLE   = 0x01;
        const IK_DATA  = 0x02;
        const IK_DATA_BIBED_DATA  = 0x04;
        const EXPLODABLE = 0x08;
        const IK_DATA_IS_HEAD = 0x10;
        const IK_DATA_HEADTRACKING = 0x20;
        const TO_HIT_CHANCE_ABSOLUTE = 0x40;
    }
}

impl RecordCollection for BodyPart {
    fn parse_next<'b>(
        parser: &mut RecordParser<'_, 'b>,
    ) -> Result<Option<Self>, RecordParseError<'b>> {
        let part_name: String = match parser.try_parse(BPTN)? {
            Some(value) => value,
            None => return Ok(None),
        };
        let part_node: String = parser.parse(BPNN)?;
        let vats_target: String = parser.parse(BPNT)?;
        let ik_data_start_node: String = parser.parse(BPNT)?;
        let bpnd: BPND = parser.parse(BPND)?;
        let limb_replacement_mode: String = parser.parse(NAM1)?;
        let gore_effects_target_bone: String = parser.parse(NAM4)?;
        parser.skip_type(NAM5);

        Ok(Some(Self {
            part_name,
            part_node,
            vats_target,
            ik_data_start_node,
            bpnd,
            limb_replacement_mode,
            gore_effects_target_bone,
        }))
    }
}

impl RecordCollection for UnnamedBodyPart {
    fn parse_next<'b>(
        parser: &mut RecordParser<'_, 'b>,
    ) -> Result<Option<Self>, RecordParseError<'b>> {
        let part_node: String = match parser.try_parse(BPNN)? {
            Some(value) => value,
            None => return Ok(None),
        };
        let vats_target: String = parser.parse(BPNT)?;
        let ik_data_start_node: String = parser.parse(BPNT)?;
        let bpnd: BPND = parser.parse(BPND)?;
        let limb_replacement_mode: String = parser.parse(NAM1)?;
        let gore_effects_target_bone: String = parser.parse(NAM4)?;
        parser.skip_type(NAM5);

        Ok(Some(Self {
            part_node,
            vats_target,
            ik_data_start_node,
            bpnd,
            limb_replacement_mode,
            gore_effects_target_bone,
        }))
    }
}

impl FromRecordBytes for BPND {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, damage_multiplier) = le_f32(input)?;
        let (input, flags) = BodyPartFlag::parse(input)?;
        let (input, part_type) = enum_value::<PartType>(input)?;
        let (input, health_percent) = u8(input)?;
        let (input, actor_value) = enum_value::<ActorValue>(input)?;
        let (input, to_hit_chance) = u8(input)?;
        let (input, ex_explosion_chance_percent) = u8(input)?;
        let (input, ex_debris_count) = le_u16(input)?;
        let (input, ex_debris) = NTypedFormId::parse(input)?;
        let (input, ex_explosion) = NTypedFormId::parse(input)?;
        let (input, tracking_max_angle) = le_f32(input)?;
        let (input, ex_debris_scale) = le_f32(input)?;
        let (input, sv_debris_count) = le_u16(input)?;
        let (input, sv_debris) = NTypedFormId::parse(input)?;
        let (input, sv_explosion) = NTypedFormId::parse(input)?;
        let (input, sv_debris_scale) = le_f32(input)?;
        let (input, gr_translate) = Vector3::parse(input)?;
        let (input, gr_rotation) = Vector3::parse(input)?;
        let (input, sv_impact_dataset) = NTypedFormId::parse(input)?;
        let (input, ex_impact_dataset) = NTypedFormId::parse(input)?;
        let (input, sv_decal_count) = u8(input)?;
        let (input, ex_decal_count) = u8(input)?;
        let (input, limb_replacement_scale) = le_f32(input)?;

        Ok((
            input,
            Self {
                damage_multiplier,
                flags,
                part_type,
                health_percent,
                actor_value,
                to_hit_chance,
                ex_explosion_chance_percent,
                ex_debris_count,
                ex_debris,
                ex_explosion,
                tracking_max_angle,
                ex_debris_scale,
                sv_debris_count,
                sv_debris,
                sv_explosion,
                sv_debris_scale,
                gr_translate,
                gr_rotation,
                sv_impact_dataset,
                ex_impact_dataset,
                sv_decal_count,
                ex_decal_count,
                limb_replacement_scale,
            },
        ))
    }
}

impl FromRecordBytes for BodyPartFlag {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum PartType {
    Torso = 0,
    Head1 = 1,
    Head2 = 2,
    LeftArm1 = 3,
    LeftArm2 = 4,
    RightArm1 = 5,
    RightArm2 = 6,
    LeftLeg1 = 7,
    LeftLeg2 = 8,
    LeftLeg3 = 9,
    RightLeg1 = 10,
    RightLeg2 = 11,
    RightLeg3 = 12,
    Brain = 13,
    Weapon = 14,
}
