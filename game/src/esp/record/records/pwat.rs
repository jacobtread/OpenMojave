use bitflags::bitflags;

use crate::esp::{
    record::records::prelude::*,
    record::sub::{model::ModelData, object_bounds::ObjectBounds, DNAM, EDID, OBND},
    shared::{EditorId, TypedFormId},
};

#[derive(Debug)]
pub struct PWAT {
    pub editor_id: EditorId,
    pub object_bounds: ObjectBounds,
    pub model_data: ModelData,
    pub dnam: DNAM,
}

impl Record for PWAT {
    const TYPE: RecordType = RecordType::from_value(b"PWAT");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let object_bounds: ObjectBounds = parser.parse(OBND)?;
        let model_data: ModelData = ModelData::parse_first(parser)?
            .ok_or_else(|| RecordParseError::Custom("PWAT missing model data".to_string()))?;
        let dnam: DNAM = parser.parse(DNAM)?;

        Ok(Self {
            editor_id,
            object_bounds,
            model_data,
            dnam,
        })
    }
}

#[derive(Debug)]
pub struct DNAM {
    pub flags: DNAMFlags,
    pub water: TypedFormId<() /* WATR */>,
}

impl FromRecordBytes for DNAM {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((DNAMFlags::parse, TypedFormId::parse)),
            |(flags, water)| Self { flags, water },
        )(input)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct DNAMFlags: u32 {
        const REFLECTS   = 0x00000001;
        const REFLECTS_ACTORS  = 0x00000002;
        const REFLECTS_LAND  = 0x00000004;
        const REFLECTS_LOD_LAND  = 0x00000008;
        const REFLECTS_LOD_BUILDINGS  = 0x00000010;
        const REFLECTS_TREES  = 0x00000020;
        const REFLECTS_SKY  = 0x00000040;
        const REFLECTS_DYNAMIC_OBJECTS  = 0x00000080;
        const REFLECTS_DEAD_BODIES  = 0x00000100;
        const REFRACTS  = 0x00000200;
        const REFRACTS_ACTORS  = 0x00000400;
        const REFRACTS_LAND  = 0x00000800;
        const U1  = 0x00001000;
        const U2  = 0x00002000;
        const U3  = 0x00004000;
        const U4  = 0x00008000;
        const REFRACTS_DYNAMIC_OBJECTS = 0x00010000;
        const REFRACTS_DEAD_BODIES = 0x00020000;
        const SILHOUETTE_REFLECTIONS = 0x00040000;
        const U5  = 0x00080000;
        const U6  = 0x00100000;
        const U7  = 0x00200000;
        const U8  = 0x00400000;
        const U9  = 0x00800000;
        const U10  = 0x01000000;
        const U11  = 0x02000000;
        const U12  = 0x03000000;
        const U13  = 0x08000000;
        const DEPTH = 0x10000000;
        const OBJECT_TEXTURE_COORDINATES = 0x20000000;
        const U14 = 0x40000000;
        const NO_UNDERWATER_FOG = 0x80000000;
    }
}

impl FromRecordBytes for DNAMFlags {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(le_u32, Self::from_bits_retain)(input)
    }
}
