use bitflags::bitflags;
use nom::{
    branch::Alt,
    combinator::map,
    multi::length_count,
    number::complete::{le_i32, le_u32, u8},
    sequence::tuple,
    IResult,
};

use crate::esp::{
    record::{FromRecordBytes, RecordParseError, RecordParser},
    shared::{FormId, String32},
};

use super::{
    MO2S, MO2T, MO3S, MO3T, MO4S, MO4T, MOD2, MOD3, MOD4, MODB, MODD, MODL, MODS, MODT, MOSD,
};

#[derive(Debug)]
pub struct ModelData {
    pub model_file_name: String,
    pub alternative_textures: Option<AlternateTextures>,
    pub facegen_model_flags: Option<MODDFlags>,
}

impl ModelData {
    pub fn parse_first<'b>(
        parser: &mut RecordParser<'_, 'b>,
    ) -> Result<Option<Self>, RecordParseError<'b>> {
        let model_file_name = match parser.try_parse::<String>(MODL)? {
            Some(value) => value,
            None => return Ok(None),
        };
        parser.skip_type(MODB);
        parser.skip_type(MODT);
        let alternative_textures = parser.try_parse::<AlternateTextures>(MODS)?;
        let facegen_model_flags = parser.try_parse::<MODDFlags>(MODD)?;

        Ok(Some(ModelData {
            model_file_name,
            alternative_textures,
            facegen_model_flags,
        }))
    }

    pub fn parse_second<'b>(
        parser: &mut RecordParser<'_, 'b>,
    ) -> Result<Option<Self>, RecordParseError<'b>> {
        let model_file_name = match parser.try_parse::<String>(MOD2)? {
            Some(value) => value,
            None => return Ok(None),
        };
        parser.skip_type(MO2T);
        let alternative_textures = parser.try_parse::<AlternateTextures>(MO2S)?;
        Ok(Some(ModelData {
            model_file_name,
            alternative_textures,
            facegen_model_flags: None,
        }))
    }

    pub fn parse_third<'b>(
        parser: &mut RecordParser<'_, 'b>,
    ) -> Result<Option<Self>, RecordParseError<'b>> {
        let model_file_name = match parser.try_parse::<String>(MOD3)? {
            Some(value) => value,
            None => return Ok(None),
        };
        parser.skip_type(MO3T);
        let alternative_textures = parser.try_parse::<AlternateTextures>(MO3S)?;
        let facegen_model_flags = parser.try_parse::<MODDFlags>(MOSD)?;

        Ok(Some(ModelData {
            model_file_name,
            alternative_textures,
            facegen_model_flags,
        }))
    }

    pub fn parse_fourth<'b>(
        parser: &mut RecordParser<'_, 'b>,
    ) -> Result<Option<Self>, RecordParseError<'b>> {
        let model_file_name = match parser.try_parse::<String>(MOD4)? {
            Some(value) => value,
            None => return Ok(None),
        };

        parser.skip_type(MO4T);
        let alternative_textures = parser.try_parse::<AlternateTextures>(MO4S)?;
        Ok(Some(ModelData {
            model_file_name,
            alternative_textures,
            facegen_model_flags: None,
        }))
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct MODDFlags: u8 {
        const HEAD       = 0x01;
        const TORSO      = 0x02;
        const RIGHT_HAND = 0x04;
        const LEFT_HAND  = 0x08;
    }
}

impl FromRecordBytes for MODDFlags {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}

#[derive(Debug)]
pub struct AlternateTextures(pub Vec<AlternateTexture>);

impl FromRecordBytes for AlternateTextures {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(length_count(le_u32, AlternateTexture::parse), Self)(input)
    }
}

#[derive(Debug)]
pub struct AlternateTexture {
    pub name_3d: String,
    pub new_texture: FormId,
    pub index_3d: i32,
}

impl FromRecordBytes for AlternateTexture {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((String32::parse, FormId::parse, le_i32)),
            |(name_3d, new_texture, index_3d)| Self {
                name_3d: name_3d.0,
                new_texture,
                index_3d,
            },
        )(input)
    }
}
