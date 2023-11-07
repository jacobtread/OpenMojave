use super::{
    cams::CAMS,
    prelude::{condition::CTDA, *},
};

/// Camera Path
#[derive(Debug)]
pub struct CPTH {
    pub editor_id: EditorId,
    pub conditions: Vec<CTDA>,
    pub related_camera_paths: Option<RelatedCameraPaths>,
    pub camera_zoom: CameraZoom,
    pub camera_shots: Vec<TypedFormId<CAMS>>,
}

impl Record for CPTH {
    const TYPE: RecordType = CPTH;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let conditions: Vec<CTDA> = parser.try_parse_many(CTDA)?;
        let related_camera_paths: Option<RelatedCameraPaths> = parser.try_parse(ANAM)?;
        let camera_zoom: CameraZoom = parser.parse(DATA)?;
        let camera_shots: Vec<TypedFormId<CAMS>> = parser.try_parse_many(SNAM)?;

        Ok(Self {
            editor_id,
            conditions,
            related_camera_paths,
            camera_zoom,
            camera_shots,
        })
    }
}

#[derive(Debug)]
pub struct RelatedCameraPaths {
    pub parent_camera_path: NTypedFormId<CPTH>,
    pub previous_sibling_camera_path: NTypedFormId<CPTH>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum CameraZoom {
    Default = 0,
    Disable = 1,
    ShotList = 2,
}

impl FromRecordBytes for RelatedCameraPaths {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((NTypedFormId::parse, NTypedFormId::parse)),
            |(parent_camera_path, previous_sibling_camera_path)| Self {
                parent_camera_path,
                previous_sibling_camera_path,
            },
        )(input)
    }
}

impl FromRecordBytes for CameraZoom {
    #[inline]
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        enum_value(input)
    }
}
