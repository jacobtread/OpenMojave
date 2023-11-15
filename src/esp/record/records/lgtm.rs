use super::prelude::*;

/// Lighting Template
#[derive(Debug)]
pub struct LGTM {
    pub editor_id: EditorId,
    pub data: LightingTemplateData,
}

impl Record for LGTM {
    const TYPE: RecordType = RecordType::new(b"LGTM");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let data: LightingTemplateData = parser.parse(DATA)?;
        Ok(Self { editor_id, data })
    }
}

#[derive(Debug)]
pub struct LightingTemplateData {
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

impl FromRecordBytes for LightingTemplateData {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
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
