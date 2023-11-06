use bitflags::bitflags;
use fyrox::core::algebra::Vector3;
use nom::{bytes::complete::take, combinator::rest};

pub use super::prelude::*;
use crate::esp::{
    record::sub::{
        script::Script, DATA, EDID, INAM, NAME, TNAM, XADP, XAPR, XATO, XCLP, XCNT, XDCR, XEMI,
        XESP, XEZN, XHLP, XIBS, XLCM, XLKR, XMBR, XMRC, XOWN, XPPA, XPRD, XRDS, XRGB, XRGD, XRNK,
        XSCL,
    },
    shared::{EditorId, FormId, TypedFormId, RGBA},
};

/// Placed NPC
#[derive(Debug)]
pub struct ACHR {
    pub editor_id: EditorId,
    pub base: TypedFormId<() /* NPC_ */>,
    pub encounter_zone: Option<TypedFormId<() /* ECZN */>>,
    pub idle_time: f32,
    pub idle: TypedFormId<() /* IDLE / null */>,
    pub embedded_script: Script,
    pub topic: TypedFormId<() /* DIAL or null */>,
    pub level_modifier: Option<i32>,
    /// Ownership data. FormID of a FACT, ACHR, CREA or NPC_ record.
    pub owner: Option<FormId>,
    pub faction_rank: Option<i32>,
    pub merchant_container: Option<TypedFormId<() /* REFR */>>,
    pub count: Option<i32>,
    pub radius: Option<f32>,
    pub health: Option<f32>,
    pub decals: Vec<XDCR>,
    /// FormID of a REFR, ACRE, ACHR, PGRE or PMIS record.
    pub linked_ref: Option<FormId>,
    pub linked_ref_color: Option<XCLP>,
    pub flags: Option<XAPDFlags>,
    pub activate_parent_ref: Vec<XAPR>,
    pub activation_prompt: Option<String>,
    pub enable_parent: Option<XESP>,
    /// FormID of a LIGH or REGN record.
    pub emittance: Option<FormId>,
    pub multibound_ref: Option<TypedFormId<() /* REFR */>>,
    pub ignored_by_sandbox: bool,
    pub scale: Option<f32>,
    pub position_rotation: PositionRotation,
}

impl Record for ACHR {
    const TYPE: RecordType = RecordType::new(b"ACHR");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let base: TypedFormId<()> = parser.parse(NAME)?;
        let encounter_zone: Option<TypedFormId<()>> = parser.try_parse(XEZN)?;

        // Ragdoll data
        parser.skip_type(XRGD);
        // Ragdoll biped data
        parser.skip_type(XRGB);

        let idle_time: f32 = parser.parse(XPRD)?;

        // Patrol script marker
        parser.require_type(XPPA)?;

        let idle: TypedFormId<()> = parser.parse(INAM)?;
        let embedded_script: Script = Script::require_parse_next(parser)?;
        let topic: TypedFormId<()> = parser.parse(TNAM)?;
        let level_modifier: Option<i32> = parser.try_parse(XLCM)?;
        let owner: Option<FormId> = parser.try_parse(XOWN)?;
        let faction_rank: Option<i32> = parser.try_parse(XRNK)?;
        let merchant_container: Option<TypedFormId<()>> = parser.try_parse(XMRC)?;
        let count: Option<i32> = parser.try_parse(XCNT)?;
        let radius: Option<f32> = parser.try_parse(XRDS)?;
        let health: Option<f32> = parser.try_parse(XHLP)?;
        let decals: Vec<XDCR> = parser.try_parse_many(XDCR)?;
        let linked_ref: Option<FormId> = parser.try_parse(XLKR)?;
        let linked_ref_color: Option<XCLP> = parser.try_parse(XCLP)?;
        let flags: Option<XAPDFlags> = parser.try_parse(XADP)?;
        let activate_parent_ref: Vec<XAPR> = parser.try_parse_many(XAPR)?;
        let activation_prompt: Option<String> = parser.try_parse(XATO)?;
        let enable_parent: Option<XESP> = parser.try_parse(XESP)?;
        let emittance: Option<FormId> = parser.try_parse(XEMI)?;
        let multibound_ref: Option<TypedFormId<()>> = parser.try_parse(XMBR)?;
        let ignored_by_sandbox: bool = parser.try_next(XIBS).is_some();
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

#[derive(Debug)]
pub struct XDCR {
    pub reference: TypedFormId<() /* REFR */>,
}

impl FromRecordBytes for XDCR {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(tuple((TypedFormId::parse, rest)), |(reference, _)| Self {
            reference,
        })(input)
    }
}

#[derive(Debug)]
pub struct XCLP {
    pub start: RGBA,
    pub end: RGBA,
}

impl FromRecordBytes for XCLP {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(tuple((RGBA::parse, RGBA::parse)), |(start, end)| Self {
            start,
            end,
        })(input)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct XAPDFlags : u8 {
        const PARENT_ACTIVATE_ONLY   = 0x01;

    }
}

impl FromRecordBytes for XAPDFlags {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}

/// Activate Parent Ref
#[derive(Debug)]
pub struct XAPR {
    /// FormID of a REFR, ACRE, ACHR, PGRE or PMIS record.
    pub reference: FormId,
    pub delay: f32,
}

impl FromRecordBytes for XAPR {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(tuple((FormId::parse, le_f32)), |(reference, delay)| Self {
            reference,
            delay,
        })(input)
    }
}

#[derive(Debug)]
pub struct XESP {
    /// FormID of a PLYR, REFR, ACRE, ACHR, PGRE or PMIS record.
    pub reference: FormId,
    pub flags: XESPFlags,
}

impl FromRecordBytes for XESP {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((FormId::parse, XESPFlags::parse, take(3usize))),
            |(reference, flags, _)| Self { reference, flags },
        )(input)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct XESPFlags : u8 {
        const SET_ENABLE_STATE_TO_OPPOSITE_OF_PARENT   = 0x01;
        const POP_IN = 0x02;
    }
}

impl FromRecordBytes for XESPFlags {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}

#[derive(Debug)]
pub struct PositionRotation {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
}

impl FromRecordBytes for PositionRotation {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((Vector3::parse, Vector3::parse)),
            |(position, rotation)| Self { position, rotation },
        )(input)
    }
}
