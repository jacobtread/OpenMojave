use fyrox::core::algebra::Vector2;

use crate::esp::{
    record::{
        sub::{
            model::ModelData, object_bounds::ObjectBounds, BNAM, CNAM, EDID, ICON, MICO, OBND, SNAM,
        },
        Collection,
    },
    shared::EditorId,
};

pub use super::prelude::*;

#[derive(Debug)]
pub struct TREE {
    pub editor_id: EditorId,
    pub object_bounds: ObjectBounds,
    pub model_data: ModelData,
    pub large_icon_file_name: String,
    pub small_icon_file_name: String,
    pub speed_tree_seeds: Vec<u32>,
    pub tree_data: TreeData,
    /// Width, Height
    pub billboard_size: Vector2<f32>,
}

impl Record for TREE {
    const TYPE: RecordType = RecordType::new(b"TREE");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let object_bounds: ObjectBounds = parser.parse(OBND)?;
        let model_data: ModelData = ModelData::parse_first(parser)?
            .ok_or_else(|| RecordParseError::Custom("TREE missing model data".to_string()))?;
        let large_icon_file_name: String = parser.parse(ICON)?;
        let small_icon_file_name: String = parser.parse(MICO)?;
        let speed_tree_seeds: Vec<u32> = parser.parse::<Collection<u32>>(SNAM)?.into_inner();
        let tree_data: TreeData = parser.parse(CNAM)?;
        let billboard_size: Vector2<f32> = parser.parse(BNAM)?;

        Ok(Self {
            editor_id,
            object_bounds,
            model_data,
            large_icon_file_name,
            small_icon_file_name,
            speed_tree_seeds,
            tree_data,
            billboard_size,
        })
    }
}

#[derive(Debug)]
pub struct TreeData {
    pub leaf_curvature: f32,
    pub min_leaf_angle: f32,
    pub max_leaf_angle: f32,
    pub branch_dimming_value: f32,
    pub leaf_dimming_value: f32,
    pub shadow_radius: i32,
    pub rock_speed: f32,
    pub rustle_speed: f32,
}

impl FromRecordBytes for TreeData {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((
                le_f32, le_f32, le_f32, le_f32, le_f32, le_i32, le_f32, le_f32,
            )),
            |(
                leaf_curvature,
                min_leaf_angle,
                max_leaf_angle,
                branch_dimming_value,
                leaf_dimming_value,
                shadow_radius,
                rock_speed,
                rustle_speed,
            )| Self {
                leaf_curvature,
                min_leaf_angle,
                max_leaf_angle,
                branch_dimming_value,
                leaf_dimming_value,
                shadow_radius,
                rock_speed,
                rustle_speed,
            },
        )(input)
    }
}
