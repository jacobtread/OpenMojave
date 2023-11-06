use super::{prelude::*, scpt::SCPT, soun::SOUN};
use crate::esp::record::sub::{
    destruction::DestructionData, model::ModelData, object_bounds::ObjectBounds,
};

/// Door
#[derive(Debug)]
pub struct DOOR {
    pub editor_id: EditorId,
    pub object_bounds: ObjectBounds,
    pub name: Option<String>,
    pub model_data: ModelData,
    pub script: Option<TypedFormId<SCPT>>,
    pub destruction_data: Option<DestructionData>,
    pub sound_open: Option<TypedFormId<SOUN>>,
    pub sound_close: Option<TypedFormId<SOUN>>,
    pub sound_looping: Option<TypedFormId<SOUN>>,
    pub flags: Flags,
}

impl Record for DOOR {
    const TYPE: RecordType = RecordType::new(b"DOOR");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let object_bounds: ObjectBounds = parser.parse(OBND)?;
        let name: Option<String> = parser.try_parse(FULL)?;
        let model_data: ModelData = ModelData::parse_first(parser)?
            .ok_or_else(|| RecordParseError::Custom("DOOR missing model data".to_string()))?;
        let script: Option<TypedFormId<SCPT>> = parser.try_parse(SCRI)?;
        let destruction_data: Option<DestructionData> = DestructionData::parse_next(parser)?;
        let sound_open: Option<TypedFormId<SOUN>> = parser.try_parse(SNAM)?;
        let sound_close: Option<TypedFormId<SOUN>> = parser.try_parse(ANAM)?;
        let sound_looping: Option<TypedFormId<SOUN>> = parser.try_parse(BNAM)?;
        let flags: Flags = parser.parse(FNAM)?;

        Ok(Self {
            editor_id,
            object_bounds,
            name,
            model_data,
            script,
            destruction_data,
            sound_open,
            sound_close,
            sound_looping,
            flags,
        })
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct Flags: u8 {
        const UNKNOWN   = 0x01;
        const AUTOMATIC_DOOR  = 0x02;
        const HIDDEN  = 0x04;
        const MINIMAL_USE  = 0x08;
        const SLIDING_DOOR  = 0x10;
    }
}

impl FromRecordBytes for Flags {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}
