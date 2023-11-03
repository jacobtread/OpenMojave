use bitflags::bitflags;
use nom::{
    combinator::map,
    number::complete::{le_i32, le_u32, u8},
    sequence::tuple,
    IResult,
};

use crate::esp::{
    record::{FromRecordBytes, RecordCollection},
    shared::{FormId, TypedFormId},
};

use super::{DEST, DMDL, DMDT, DSTD, DSTF};

#[derive(Debug)]
pub struct Destruction {
    pub header: DEST,
    pub stages: Vec<DestructionStage>,
}

impl RecordCollection for Destruction {
    fn parse_next<'b>(
        parser: &mut crate::esp::record::RecordParser<'_, 'b>,
    ) -> Result<Option<Self>, crate::esp::record::RecordParseError<'b>> {
        let header = match parser.try_parse::<DEST>(DEST)? {
            Some(value) => value,
            None => return Ok(None),
        };
        let stages = parser.parse_collection::<DestructionStage>()?;

        Ok(Some(Self { header, stages }))
    }
}

#[derive(Debug)]
pub struct DEST {
    pub health: i32,
    pub count: u8,
    pub flags: DESTFlags,
}

impl FromRecordBytes for DEST {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((le_i32, u8, DESTFlags::parse, le_u32)),
            |(health, count, flags, _)| Self {
                health,
                count,
                flags,
            },
        )(input)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct DESTFlags: u8 {
        const VATS_TARGETABLE = 0x01;
    }
}

impl FromRecordBytes for DESTFlags {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}

#[derive(Debug)]
pub struct DestructionStage {
    pub stage_data: DSTD,
    pub stage_model_file_name: Option<String>,
}

impl RecordCollection for DestructionStage {
    fn parse_next<'b>(
        parser: &mut crate::esp::record::RecordParser<'_, 'b>,
    ) -> Result<Option<Self>, crate::esp::record::RecordParseError<'b>> {
        let stage_data = match parser.try_parse::<DSTD>(DSTD)? {
            Some(value) => value,
            None => return Ok(None),
        };
        let stage_model_file_name = parser.try_parse::<String>(DMDL)?;
        parser.skip_type(DMDT);
        parser.skip_type(DSTF);

        Ok(Some(Self {
            stage_data,
            stage_model_file_name,
        }))
    }
}

#[derive(Debug)]
pub struct DSTD {
    pub health_percent: u8,
    pub index: u8,
    pub damage_stage: u8,
    pub flags: DSTDFlags,
    pub self_damage_per_second: i32,
    pub explosion: TypedFormId<() /* EXPL */>,
    pub debris: TypedFormId<() /* DEBR */>,
    pub debris_count: i32,
}

impl FromRecordBytes for DSTD {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((
                u8,
                u8,
                u8,
                DSTDFlags::parse,
                le_i32,
                TypedFormId::parse,
                TypedFormId::parse,
                le_i32,
            )),
            |(
                health_percent,
                index,
                damage_stage,
                flags,
                self_damage_per_second,
                explosion,
                debris,
                debris_count,
            )| Self {
                health_percent,
                index,
                damage_stage,
                flags,
                self_damage_per_second,
                explosion,
                debris,
                debris_count,
            },
        )(input)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct DSTDFlags: u8 {
        const CAP_DAMAGE = 0x01;
        const DISABLE = 0x02;
        const DESTROY = 0x04;
    }
}

impl FromRecordBytes for DSTDFlags {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}
