use super::{
    achr::{LinkedRefColor, PositionRotation, XAPDFlags, XAPR, XDCR, XESP},
    dial::DIAL,
    eczn::ECZN,
    idle::IDLE,
    npc::NPC_,
    prelude::*,
    refr::REFR,
};
use crate::esp::record::sub::script::Script;

/// Placed Creature
#[derive(Debug)]
pub struct ACRE {
    pub editor_id: EditorId,
    pub base: TypedFormId<NPC_>,
    pub encounter_zone: Option<TypedFormId<ECZN>>,
    pub idle_time: f32,
    pub idle: NTypedFormId<IDLE>,
    pub embedded_script: Script,
    pub topic: NTypedFormId<DIAL>,
    pub level_modifier: Option<i32>,
    /// Ownership data. FormID of a FACT, ACHR, CREA or NPC_ record.
    pub owner: Option<FormId>,
    pub faction_rank: Option<i32>,
    pub merchant_container: Option<TypedFormId<REFR>>,
    pub count: Option<i32>,
    pub radius: Option<f32>,
    pub health: Option<f32>,
    pub decals: Vec<XDCR>,
    /// FormID of a REFR, ACRE, ACHR, PGRE or PMIS record.
    pub linked_ref: Option<FormId>,
    pub linked_ref_color: Option<LinkedRefColor>,
    pub flags: Option<XAPDFlags>,
    pub activate_parent_ref: Vec<XAPR>,
    pub activation_prompt: Option<String>,
    pub enable_parent: Option<XESP>,
    /// FormID of a LIGH or REGN record.
    pub emittance: Option<FormId>,
    pub multibound_ref: Option<TypedFormId<REFR>>,
    pub ignored_by_sandbox: bool,
    pub scale: Option<f32>,
    pub position_rotation: PositionRotation,
}

impl Record for ACRE {
    const TYPE: RecordType = ACRE;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let base: TypedFormId<_> = parser.parse(NAME)?;
        let encounter_zone: Option<TypedFormId<_>> = parser.try_parse(XEZN)?;

        // Ragdoll data
        parser.skip_type(XRGD);
        // Ragdoll biped data
        parser.skip_type(XRGB);

        let idle_time: f32 = parser.parse(XPRD)?;

        // Patrol script marker
        parser.require_type(XPPA)?;

        let idle: TypedFormId<_> = parser.parse(INAM)?;
        let embedded_script: Script = Script::require_parse_next(parser)?;
        let topic: TypedFormId<_> = parser.parse(TNAM)?;
        let level_modifier: Option<i32> = parser.try_parse(XLCM)?;
        let owner: Option<FormId> = parser.try_parse(XOWN)?;
        let faction_rank: Option<i32> = parser.try_parse(XRNK)?;
        let merchant_container: Option<TypedFormId<_>> = parser.try_parse(XMRC)?;
        let count: Option<i32> = parser.try_parse(XCNT)?;
        let radius: Option<f32> = parser.try_parse(XRDS)?;
        let health: Option<f32> = parser.try_parse(XHLP)?;
        let decals: Vec<XDCR> = parser.try_parse_many(XDCR)?;
        let linked_ref: Option<FormId> = parser.try_parse(XLKR)?;
        let linked_ref_color: Option<LinkedRefColor> = parser.try_parse(XCLP)?;
        let flags: Option<XAPDFlags> = parser.try_parse(XADP)?;
        let activate_parent_ref: Vec<XAPR> = parser.try_parse_many(XAPR)?;
        let activation_prompt: Option<String> = parser.try_parse(XATO)?;
        let enable_parent: Option<XESP> = parser.try_parse(XESP)?;
        let emittance: Option<FormId> = parser.try_parse(XEMI)?;
        let multibound_ref: Option<TypedFormId<_>> = parser.try_parse(XMBR)?;
        let ignored_by_sandbox: bool = parser.next_if(XIBS).is_some();
        let scale: Option<f32> = parser.try_parse(XSCL)?;
        let position_rotation: PositionRotation = parser.parse(DATA)?;

        Ok(Self {
            editor_id,
            base,
            encounter_zone,

            idle_time,
            idle,
            embedded_script,
            topic,
            level_modifier,
            owner,
            faction_rank,
            merchant_container,
            count,
            radius,
            health,
            decals,
            linked_ref,
            linked_ref_color,
            flags,
            activate_parent_ref,
            activation_prompt,
            enable_parent,
            emittance,
            multibound_ref,
            ignored_by_sandbox,
            scale,
            position_rotation,
        })
    }
}
