use super::prelude::*;
use crate::esp::{
    record::sub::{model::ModelData, object_bounds::ObjectBounds},
    shared::EditorId,
};

/// Addon Node
#[derive(Debug)]
pub struct ADDN {
    pub editor_id: EditorId,
    pub object_bounds: ObjectBounds,
    pub model_data: ModelData,
    pub node_index: i32,
    pub data: ADDNData,
}

impl Record for ADDN {
    const TYPE: RecordType = RecordType::new(b"ADDN");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}

#[derive(Debug)]
pub struct ADDNData {
    pub master_particle_system_cap: u16,
    pub unknown: u16,
}

impl FromRecordBytes for ADDNData {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((le_u16, le_u16)),
            |(master_particle_system_cap, unknown)| Self {
                master_particle_system_cap,
                unknown,
            },
        )(input)
    }
}
