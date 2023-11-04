use crate::esp::{
    record::{
        sub::{
            destruction::DestructionData, model::ModelData, object_bounds::ObjectBounds, DATA,
            EDID, FULL, OBND, SNAM,
        },
        Record, RecordCollection, RecordParseError, RecordParser, RecordType,
    },
    shared::{EditorId, TypedFormId},
};

use super::soun::SOUN;

/// Moveable Static
#[derive(Debug)]
pub struct MSTT {
    pub editor_id: EditorId,
    pub object_bounds: ObjectBounds,
    pub name: Option<String>,
    pub model_data: ModelData,
    pub destruction_data: Option<DestructionData>,
    pub unknown: u8,
    pub sound: Option<TypedFormId<SOUN>>,
}

impl Record for MSTT {
    const TYPE: RecordType = RecordType::new(b"MSTT");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let object_bounds: ObjectBounds = parser.parse(OBND)?;
        let name: Option<String> = parser.try_parse(FULL)?;
        let model_data: ModelData = ModelData::parse_first(parser)?
            .ok_or_else(|| RecordParseError::Custom("MSTT missing model data".to_string()))?;
        let destruction_data: Option<DestructionData> = DestructionData::parse_next(parser)?;
        let unknown: u8 = parser.parse(DATA)?;
        let sound: Option<TypedFormId<SOUN>> = parser.try_parse(SNAM)?;
        Ok(Self {
            editor_id,
            object_bounds,
            name,
            model_data,
            destruction_data,
            unknown,
            sound,
        })
    }
}
