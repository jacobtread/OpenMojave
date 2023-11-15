use super::{clmt::CLMT, eczn::ECZN, imgs::IMGS, musc::MUSC, prelude::*, watr::WATR};

/// Worldspace
#[derive(Debug)]
pub struct WRLD {
    pub editor_id: EditorId,
    pub name: Option<String>,
    pub encounter_zone: Option<TypedFormId<ECZN>>,
    pub parent_worldspace: Option<TypedFormId<WRLD>>,
    pub parent_worldspace_flags: u16,
    pub climate: Option<TypedFormId<CLMT>>,
    pub water: Option<TypedFormId<WATR>>,
    pub lod_water_type: Option<TypedFormId<WATR>>,
    pub lod_water_height: Option<f32>,
    pub land_data: Option<()>,
    pub large_icon_file_name: Option<String>,
    pub small_icon_file_name: Option<String>,
    pub map_data: Option<()>,
    pub world_map_offset_data: (),
    pub image_space: Option<TypedFormId<IMGS>>,
    pub flags: u8,
    pub min_object_bounds: (),
    pub max_object_bonds: (),
    pub music: Option<TypedFormId<MUSC>>,
    pub canopy_shadow: String,
    pub water_noise_texture: String,
    pub swapped_impact: (),
    pub footstep_material: Option<()>,
    pub offset_data: Option<Vec<u32>>,
    // TODO:
}

impl Record for WRLD {
    const TYPE: RecordType = RecordType::new(b"WRLD");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        todo!()
    }
}
