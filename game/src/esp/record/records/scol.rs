use fyrox::core::algebra::Vector3;
use nom::{combinator::map, number::complete::le_f32, sequence::tuple};

use crate::esp::{
    record::{
        sub::{model::ModelData, object_bounds::ObjectBounds, DATA, EDID, OBND, ONAM},
        FromRecordBytes, Record, RecordCollection, RecordParseError, RecordParser, RecordType,
    },
    shared::{EditorId, FormId},
};

/// Static Collection
#[derive(Debug)]
pub struct SCOL {
    pub editor_id: EditorId,
    pub object_bounds: ObjectBounds,
    pub model_data: ModelData,
    pub parts: Vec<Part>,
}

impl Record for SCOL {
    const TYPE: RecordType = RecordType::new(b"SCOL");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let object_bounds: ObjectBounds = parser.parse(OBND)?;
        let model_data: ModelData = ModelData::parse_first(parser)?
            .ok_or_else(|| RecordParseError::Custom("SCOL missing model data".to_string()))?;
        let parts: Vec<Part> = parser.parse_collection()?;
        if parts.is_empty() {
            return Err(crate::esp::record::RecordParseError::Custom(
                "Missing SCOL parts".to_string(),
            ));
        }

        Ok(Self {
            editor_id,
            object_bounds,
            model_data,
            parts,
        })
    }
}

#[derive(Debug)]
pub struct Part {
    pub stat: Option<FormId>,
    pub placements: Placements,
}

impl RecordCollection for Part {
    fn parse_next<'b>(
        parser: &mut RecordParser<'_, 'b>,
    ) -> Result<Option<Self>, RecordParseError<'b>> {
        let stat: Option<FormId> = parser.try_parse(ONAM)?;
        let placements: Placements = match parser.try_parse(DATA)? {
            Some(value) => value,
            // TODO: Error if stat is present
            None => return Ok(None),
        };

        Ok(Some(Self { stat, placements }))
    }
}

#[derive(Debug)]
pub struct Placements {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: f32,
}

impl FromRecordBytes for Placements {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(
            tuple((Vector3::parse, Vector3::parse, le_f32)),
            |(position, rotation, scale)| Self {
                position,
                rotation,
                scale,
            },
        )(input)
    }
}
