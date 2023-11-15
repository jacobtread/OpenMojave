use super::{
    aspc::ASPC, clmt::CLMT, eczn::ECZN, imgs::IMGS, lgtm::LGTM, musc::MUSC, prelude::*, regn::REGN,
    watr::WATR,
};
use crate::esp::record::take_bytes_const;

/// Cell
#[derive(Debug)]
pub struct CELL {
    pub editor_id: EditorId,
    pub name: Option<String>,
    pub flags: CellFlags,
    pub grid: Option<XCLC>,
    pub lighting: Option<XCLL>,
    pub footstep_material: Option<IMPF>,
    pub light_template: LightTemplate,
    pub water_height: Option<f32>,
    pub water_noise_texture: Option<String>,
    pub regions: Option<Vec<TypedFormId<REGN>>>,
    pub image_space: Option<TypedFormId<IMGS>>,
    pub encounter_zone: Option<TypedFormId<ECZN>>,
    pub climate: Option<TypedFormId<CLMT>>,
    pub water: Option<TypedFormId<WATR>>,
    /// Ownership data. FormID of a FACT, ACHR, CREA or NPC_ record.
    pub owner: Option<FormId>,
    pub faction_rank: Option<i32>,
    pub acoustic_space: Option<TypedFormId<ASPC>>,
    pub music_type: Option<TypedFormId<MUSC>>,
}

impl Record for CELL {
    const TYPE: RecordType = CELL;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let name: Option<String> = parser.try_parse(FULL)?;
        let flags: CellFlags = parser.parse(DATA)?;
        let grid: Option<XCLC> = parser.try_parse(XCLC)?;
        let lighting: Option<XCLL> = parser.try_parse(XCLL)?;
        let footstep_material: Option<IMPF> = parser.try_parse(IMPF)?;
        let light_template: LightTemplate = LightTemplate::require_parse_next(parser)?;
        let water_height: Option<f32> = parser.try_parse(XCLW)?;
        let water_noise_texture: Option<String> = parser.try_parse(XNAM)?;
        let regions: Option<Vec<TypedFormId<REGN>>> = parser
            .try_parse::<Repeated<TypedFormId<REGN>>>(XCLR)?
            .map(|value| value.into_inner());
        let image_space: Option<TypedFormId<IMGS>> = parser.try_parse(XCIM)?;

        parser.skip_type(XCET);

        let encounter_zone: Option<TypedFormId<ECZN>> = parser.try_parse(XEZN)?;
        let climate: Option<TypedFormId<CLMT>> = parser.try_parse(XCCM)?;
        let water: Option<TypedFormId<WATR>> = parser.try_parse(XCWT)?;
        let owner: Option<FormId> = parser.try_parse(XOWN)?;
        let faction_rank: Option<i32> = parser.try_parse(XRNK)?;
        let acoustic_space: Option<TypedFormId<ASPC>> = parser.try_parse(XCAS)?;

        parser.skip_type(XCMT);

        let music_type: Option<TypedFormId<MUSC>> = parser.try_parse(XCMO)?;

        Ok(Self {
            editor_id,
            name,
            flags,
            grid,
            lighting,
            footstep_material,
            light_template,
            water_height,
            water_noise_texture,
            regions,
            image_space,
            encounter_zone,
            climate,
            water,
            owner,
            faction_rank,
            acoustic_space,
            music_type,
        })
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct CellFlags: u8 {
        const IS_INTERIOR_CELL             = 0x01;
        const HAS_WATER                    = 0x02;
        const INVERT_FAST_TRAVEL_BEHAVIOUR = 0x04;
        const NO_LOD_WATER                 = 0x08;
        const U1                           = 0x10;
        const PUBLIC_PLACE                 = 0x20;
        const HAND_CHANGED                 = 0x40;
        const BEHAVE_LIKE_EXTERIOR         = 0x80;
    }
}

#[derive(Debug)]
pub struct XCLC {
    pub x: i32,
    pub y: i32,
    pub force_hide_land: ForceHideLangFlags,
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct ForceHideLangFlags: u32 {
        const QUAD_1 = 0x00000001;
        const QUAD_2 = 0x00000004;
        const QUAD_3 = 0x00000008;
        const QUAD_4 = 0x00000010;
    }
}

#[derive(Debug)]
pub struct XCLL {
    pub ambient_color: RGBA,
    pub directional_color: RGBA,
    pub fog_color: RGBA,
    pub fog_near: f32,
    pub fog_far: f32,
    pub directional_rotation_xy: i32,
    pub directional_rotation_z: i32,
    pub directional_fade: f32,
    pub fog_clip_distance: f32,
    pub fog_power: f32,
}

#[derive(Debug)]
pub struct IMPF {
    pub conc_solid: [u8; 30],
    pub conc_broken: [u8; 30],
    pub metal_solid: [u8; 30],
    pub metal_hollow: [u8; 30],
    pub metal_sheet: [u8; 30],
    pub wood: [u8; 30],
    pub sand: [u8; 30],
    pub dirt: [u8; 30],
    pub grass: [u8; 30],
    pub water: [u8; 30],
}

#[derive(Debug)]
pub struct LightTemplate {
    pub template: NTypedFormId<LGTM>,
    pub inherit: LightInheritFlags,
}

impl FromRecordBytes for XCLC {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(
            tuple((le_i32, le_i32, ForceHideLangFlags::parse)),
            |(x, y, force_hide_land)| Self {
                x,
                y,
                force_hide_land,
            },
        )(input)
    }
}

impl FromRecordBytes for XCLL {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(
            tuple((
                RGBA::parse,
                RGBA::parse,
                RGBA::parse,
                le_f32,
                le_f32,
                le_i32,
                le_i32,
                le_f32,
                le_f32,
                le_f32,
            )),
            |(
                ambient_color,
                directional_color,
                fog_color,
                fog_near,
                fog_far,
                directional_rotation_xy,
                directional_rotation_z,
                directional_fade,
                fog_clip_distance,
                fog_power,
            )| Self {
                ambient_color,
                directional_color,
                fog_color,
                fog_near,
                fog_far,
                directional_rotation_xy,
                directional_rotation_z,
                directional_fade,
                fog_clip_distance,
                fog_power,
            },
        )(input)
    }
}

impl RecordCollection for LightTemplate {
    fn parse_next<'b>(
        parser: &mut RecordParser<'_, 'b>,
    ) -> Result<Option<Self>, RecordParseError<'b>> {
        let template: NTypedFormId<LGTM> = match parser.try_parse(LTMP)? {
            Some(value) => value,
            None => return Ok(None),
        };
        let inherit: LightInheritFlags = parser.parse(LNAM)?;
        Ok(Some(Self { template, inherit }))
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct LightInheritFlags: u32 {
        const AMBIENT_COLOR        = 0x00000001;
        const DIRECTIONAL_COLOR    = 0x00000004;
        const FOG_COLOR            = 0x00000008;
        const FOG_NEAR             = 0x00000010;
        const FOG_FAR              = 0x00000020;
        const DIRECTIONAL_ROTATION = 0x00000040;
        const DIRECTIONAL_FADE     = 0x00000080;
        const FOG_CLIP_DISTANCE    = 0x00000100;
        const FOG_POWER            = 0x00000200;
    }
}

impl FromRecordBytes for CellFlags {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}

impl FromRecordBytes for ForceHideLangFlags {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(le_u32, Self::from_bits_retain)(input)
    }
}

impl FromRecordBytes for LightInheritFlags {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(le_u32, Self::from_bits_retain)(input)
    }
}

#[inline]
fn take_30(input: &[u8]) -> nom::IResult<&[u8], [u8; 30]> {
    take_bytes_const(input)
}

impl FromRecordBytes for IMPF {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((
                take_30, take_30, take_30, take_30, take_30, take_30, take_30, take_30, take_30,
                take_30,
            )),
            |(
                conc_solid,
                conc_broken,
                metal_solid,
                metal_hollow,
                metal_sheet,
                wood,
                sand,
                dirt,
                grass,
                water,
            )| Self {
                conc_solid,
                conc_broken,
                metal_solid,
                metal_hollow,
                metal_sheet,
                wood,
                sand,
                dirt,
                grass,
                water,
            },
        )(input)
    }
}
