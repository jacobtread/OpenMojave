use bitflags::bitflags;
use fyrox::resource::model::Model;
use nom::{
    combinator::map,
    number::complete::{i8, le_f32, le_u32, u8},
    sequence::tuple,
    IResult,
};
use num_enum::TryFromPrimitive;
use serde_ini::parse;

use crate::esp::{
    record::{
        enum_value,
        sub::{
            actor_values::ActorValue, model::ModelData, xnam::XNAM, ATTR, CNAM, DATA, DESC, DNAM,
            EDID, ENAM, FGGA, FGGS, FGTS, FNAM, FULL, HNAM, ICON, INDX, MICO, MNAM, NAM0, NAM1,
            NAM2, ONAM, PNAM, SNAM, UNAM, VTCK, XNAM, YNAM,
        },
        Collection, FromRecordBytes, RawBytes, Record, RecordCollection, RecordParseError,
        RecordParser, RecordType,
    },
    shared::{EditorId, FormId},
};

#[derive(Debug)]
pub struct RACE {
    pub editor_id: EditorId,
    pub name: Option<String>,
    pub description: String,
    pub relations: Vec<XNAM>,
    pub data: RaceData,
    pub older: Option<FormId>,
    pub younger: Option<FormId>,
    pub voices: Voices,
    pub default_hair_styles: DefaultHairStyles,
    pub default_hair_colors: DefaultHairColors,
    pub facegen_main_clamp: f32,
    pub facegen_face_clamp: f32,
    pub male_head_parts: Vec<HeadPart>,
    pub female_head_parts: Vec<HeadPart>,
    pub male_body_parts: Vec<BodyPart>,
    pub female_body_parts: Vec<BodyPart>,
    pub hairs: Vec<FormId>,
    pub eyes: Vec<FormId>,

    pub male_facegen_geometry_symmetric: Vec<u8>,
    pub male_facegen_geometry_asymmetric: Vec<u8>,
    pub male_facegen_texture_symmetric: Vec<u8>,

    pub female_facegen_geometry_symmetric: Vec<u8>,
    pub female_facegen_geometry_asymmetric: Vec<u8>,
    pub female_facegen_texture_symmetric: Vec<u8>,
}

impl Record for RACE {
    const TYPE: RecordType = RecordType::from_value(b"RACE");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id = parser.parse::<EditorId>(EDID)?;
        let name = parser.try_parse::<String>(FULL)?;
        let description = parser.parse::<String>(DESC)?;

        let relations: Vec<XNAM> = parser.try_parse_many::<XNAM>(XNAM)?;

        let data = parser.parse::<RaceData>(DATA)?;

        let older = parser.try_parse::<FormId>(ONAM)?;
        let younger = parser.try_parse::<FormId>(YNAM)?;

        // Unknown marker
        parser.require_type(NAM2)?;

        let voices = parser.parse::<Voices>(VTCK)?;

        let default_hair_styles = parser.parse::<DefaultHairStyles>(DNAM)?;
        let default_hair_colors = parser.parse::<DefaultHairColors>(CNAM)?;

        let facegen_main_clamp = parser.parse::<f32>(PNAM)?;
        let facegen_face_clamp = parser.parse::<f32>(UNAM)?;

        parser.skip_type(ATTR);

        // Head parts marker
        parser.require_type(NAM0)?;

        // Male head part marker
        parser.require_type(MNAM)?;
        let male_head_parts: Vec<HeadPart> = parser.parse_collection::<HeadPart>()?;
        if male_head_parts.is_empty() {
            return Err(RecordParseError::Custom(
                "Missing male head part".to_string(),
            ));
        }

        // Female head part marker
        parser.require_type(FNAM)?;
        let female_head_parts: Vec<HeadPart> = parser.parse_collection::<HeadPart>()?;
        if female_head_parts.is_empty() {
            return Err(RecordParseError::Custom(
                "Missing female head part".to_string(),
            ));
        }

        // Body part marker
        parser.require_type(NAM1)?;

        // Male body part marker
        parser.require_type(MNAM)?;
        let male_body_parts: Vec<BodyPart> = parser.parse_collection::<BodyPart>()?;
        if male_body_parts.is_empty() {
            return Err(RecordParseError::Custom(
                "Missing male body part".to_string(),
            ));
        }

        // Female body part marker
        parser.require_type(FNAM)?;
        let female_body_parts: Vec<BodyPart> = parser.parse_collection::<BodyPart>()?;
        if female_body_parts.is_empty() {
            return Err(RecordParseError::Custom(
                "Missing female body part".to_string(),
            ));
        }

        let hairs: Vec<FormId> = parser.parse::<Collection<FormId>>(HNAM)?.into_inner();
        let eyes: Vec<FormId> = parser.parse::<Collection<FormId>>(ENAM)?.into_inner();

        // Male FaceGen Data Marker
        parser.require_type(MNAM)?;

        let male_facegen_geometry_symmetric: Vec<u8> = parser.parse::<RawBytes>(FGGS)?.into_inner();
        let male_facegen_geometry_asymmetric: Vec<u8> =
            parser.parse::<RawBytes>(FGGA)?.into_inner();
        let male_facegen_texture_symmetric: Vec<u8> = parser.parse::<RawBytes>(FGTS)?.into_inner();

        parser.require_type(SNAM)?;

        // Female FaceGen Data Marker
        parser.require_type(FNAM)?;

        let female_facegen_geometry_symmetric: Vec<u8> =
            parser.parse::<RawBytes>(FGGS)?.into_inner();
        let female_facegen_geometry_asymmetric: Vec<u8> =
            parser.parse::<RawBytes>(FGGA)?.into_inner();
        let female_facegen_texture_symmetric: Vec<u8> =
            parser.parse::<RawBytes>(FGTS)?.into_inner();

        parser.require_type(SNAM)?;

        Ok(Self {
            editor_id,
            name,
            description,
            relations,
            data,
            older,
            younger,
            voices,
            default_hair_styles,
            default_hair_colors,
            facegen_main_clamp,
            facegen_face_clamp,
            male_head_parts,
            female_head_parts,
            male_body_parts,
            female_body_parts,
            hairs,
            eyes,
            male_facegen_geometry_symmetric,
            male_facegen_geometry_asymmetric,
            male_facegen_texture_symmetric,
            female_facegen_geometry_symmetric,
            female_facegen_geometry_asymmetric,
            female_facegen_texture_symmetric,
        })
    }
}

#[derive(Debug)]
pub struct Voices {
    pub male: FormId,
    pub female: FormId,
}

impl FromRecordBytes for Voices {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(tuple((FormId::parse, FormId::parse)), |(male, female)| {
            Self { male, female }
        })(input)
    }
}

#[derive(Debug)]
pub struct DefaultHairStyles {
    pub male: FormId,
    pub female: FormId,
}

impl FromRecordBytes for DefaultHairStyles {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(tuple((FormId::parse, FormId::parse)), |(male, female)| {
            Self { male, female }
        })(input)
    }
}

#[derive(Debug)]
pub struct DefaultHairColors {
    pub male: DefaultHairColor,
    pub female: DefaultHairColor,
}

impl FromRecordBytes for DefaultHairColors {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((
                enum_value::<DefaultHairColor>,
                enum_value::<DefaultHairColor>,
            )),
            |(male, female)| Self { male, female },
        )(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum DefaultHairColor {
    Bleached = 0,
    Brown = 1,
    Chocolate = 2,
    Platinum = 3,
    Cornsilk = 4,
    Suede = 5,
    Pecan = 6,
    Auburn = 7,
    Ginger = 8,
    Honey = 9,
    Gold = 10,
    Rosewood = 11,
    Black = 12,
    Chestnut = 13,
    Steel = 14,
    Champagne = 15,
}

#[derive(Debug)]
pub struct RaceData {
    pub sb_1: SkillBoost,
    pub sb_2: SkillBoost,
    pub sb_3: SkillBoost,
    pub sb_4: SkillBoost,
    pub sb_5: SkillBoost,
    pub sb_6: SkillBoost,
    pub sb_7: SkillBoost,
    pub male_height: f32,
    pub female_height: f32,
    pub male_weight: f32,
    pub female_weight: f32,
    pub flags: RaceDataFlags,
}

impl FromRecordBytes for RaceData {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((
                SkillBoost::parse,
                SkillBoost::parse,
                SkillBoost::parse,
                SkillBoost::parse,
                SkillBoost::parse,
                SkillBoost::parse,
                SkillBoost::parse,
                le_f32,
                le_f32,
                le_f32,
                le_f32,
                RaceDataFlags::parse,
            )),
            |(
                sb_1,
                sb_2,
                sb_3,
                sb_4,
                sb_5,
                sb_6,
                sb_7,
                male_height,
                female_height,
                male_weight,
                female_weight,
                flags,
            )| Self {
                sb_1,
                sb_2,
                sb_3,
                sb_4,
                sb_5,
                sb_6,
                sb_7,
                male_height,
                female_height,
                male_weight,
                female_weight,
                flags,
            },
        )(input)
    }
}

#[derive(Debug)]
pub struct SkillBoost {
    pub skill: ActorValue,
    pub boost: i8,
}

impl FromRecordBytes for SkillBoost {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(tuple((enum_value::<ActorValue>, i8)), |(skill, boost)| {
            Self { skill, boost }
        })(input)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct RaceDataFlags: u32 {
        const PLAYABLE = 0x00000001;
        const UNKNOWN  = 0x00000002;
        const CHILD    = 0x00000004;
    }
}

impl FromRecordBytes for RaceDataFlags {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(le_u32, Self::from_bits_retain)(input)
    }
}

#[derive(Debug)]
pub struct HeadPart {
    pub index: Option<HeadPartIndex>,
    pub model_data: ModelData,
    pub large_icon_file_name: Option<String>,
    pub small_icon_file_name: Option<String>,
}

impl RecordCollection for HeadPart {
    fn parse_next<'b>(
        parser: &mut RecordParser<'_, 'b>,
    ) -> Result<Option<Self>, RecordParseError<'b>> {
        let index = parser.try_parse::<HeadPartIndex>(INDX)?;
        let model_data = ModelData::parse_first(parser)?;

        let (index, model_data) = match (index, model_data) {
            // There was nothing so the collection is finished
            (None, None) => return Ok(None),
            // Model is present
            (index, Some(model_data)) => (index, model_data),
            // Index was present but model was missing
            (Some(_), None) => {
                return Err(RecordParseError::Custom(
                    "Hair missing model data".to_string(),
                ))
            }
        };

        let large_icon_file_name = parser.try_parse::<String>(ICON)?;
        let small_icon_file_name = parser.try_parse::<String>(MICO)?;

        Ok(Some(Self {
            index,
            model_data,
            large_icon_file_name,
            small_icon_file_name,
        }))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum HeadPartIndex {
    Head = 0,
    Ears = 1,
    Mouth = 2,
    TeethLower = 3,
    TeethUpper = 4,
    Tounge = 5,
    LeftEye = 6,
    RightEye = 7,
}

impl FromRecordBytes for HeadPartIndex {
    #[inline]
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        enum_value(input)
    }
}

#[derive(Debug)]
pub struct BodyPart {
    pub index: Option<BodyPartIndex>,
    pub large_icon_file_name: Option<String>,
    pub small_icon_file_name: Option<String>,
    pub model_data: ModelData,
}

impl RecordCollection for BodyPart {
    fn parse_next<'b>(
        parser: &mut RecordParser<'_, 'b>,
    ) -> Result<Option<Self>, RecordParseError<'b>> {
        let index = parser.try_parse::<BodyPartIndex>(INDX)?;
        let large_icon_file_name = parser.try_parse::<String>(ICON)?;
        let small_icon_file_name = parser.try_parse::<String>(MICO)?;

        let model_data = match ModelData::parse_first(parser)? {
            Some(value) => value,
            None => {
                return if index.is_some()
                    || large_icon_file_name.is_some()
                    || small_icon_file_name.is_some()
                {
                    Err(RecordParseError::Custom(
                        "Hair missing model data".to_string(),
                    ))
                } else {
                    Ok(None)
                }
            }
        };

        Ok(Some(Self {
            index,
            model_data,
            large_icon_file_name,
            small_icon_file_name,
        }))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum BodyPartIndex {
    UpperBody = 0,
    LeftHand = 1,
    RightHand = 2,
    UpperBodyTexture = 3,
}

impl FromRecordBytes for BodyPartIndex {
    #[inline]
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        enum_value(input)
    }
}
