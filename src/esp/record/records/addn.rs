use super::prelude::*;
use crate::esp::record::sub::{model::ModelData, object_bounds::ObjectBounds};

/// Addon Node
#[derive(Debug)]
pub struct ADDN {
    pub editor_id: EditorId,
    pub object_bounds: ObjectBounds,
    pub model_data: ModelData,
    pub node_index: i32,
    pub data: AddonNodeData,
}

impl Record for ADDN {
    const TYPE: RecordType = ADDN;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let object_bounds: ObjectBounds = parser.parse(OBND)?;
        let model_data: ModelData = ModelData::require(parser)?;
        let node_index: i32 = parser.parse(DATA)?;
        let data: AddonNodeData = parser.parse(DNAM)?;
        Ok(Self {
            editor_id,
            object_bounds,
            model_data,
            node_index,
            data,
        })
    }
}

#[derive(Debug)]
pub struct AddonNodeData {
    pub master_particle_system_cap: u16,
    pub unknown: u16,
}

impl FromRecordBytes for AddonNodeData {
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
