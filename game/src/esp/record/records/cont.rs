use super::{prelude::*, scpt::SCPT, soun::SOUN};
use crate::esp::record::sub::{
    destruction::DestructionData, item::Item, model::ModelData, object_bounds::ObjectBounds,
};

/// Container
#[derive(Debug)]
pub struct CONT {
    pub editor_id: EditorId,
    pub object_bounds: ObjectBounds,
    pub name: Option<String>,
    pub model_data: ModelData,
    pub script: Option<TypedFormId<SCPT>>,
    pub items: Vec<Item>,
    pub destruction_data: Option<DestructionData>,
    pub data: Option<ContainerData>,
    pub sound_open: Option<TypedFormId<SOUN>>,
    pub sound_close: Option<TypedFormId<SOUN>>,
    pub sound_random: Option<TypedFormId<SOUN>>,
}

impl Record for CONT {
    const TYPE: RecordType = CONT;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let object_bounds: ObjectBounds = parser.parse(OBND)?;
        let name: Option<String> = parser.try_parse(FULL)?;
        let model_data: ModelData = ModelData::require(parser)?;
        let script: Option<TypedFormId<SCPT>> = parser.try_parse(SCRI)?;
        let items: Vec<Item> = parser.parse_collection()?;
        let destruction_data: Option<DestructionData> = DestructionData::parse_next(parser)?;
        let data: Option<ContainerData> = parser.try_parse(DATA)?;
        let sound_open: Option<TypedFormId<SOUN>> = parser.try_parse(SNAM)?;
        let sound_close: Option<TypedFormId<SOUN>> = parser.try_parse(QNAM)?;
        let sound_random: Option<TypedFormId<SOUN>> = parser.try_parse(RNAM)?;

        Ok(Self {
            editor_id,
            object_bounds,
            name,
            model_data,
            script,
            items,
            destruction_data,
            data,
            sound_open,
            sound_close,
            sound_random,
        })
    }
}

#[derive(Debug)]
pub struct ContainerData {
    pub flags: ContainerFlags,
    pub weight: f32,
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct ContainerFlags: u8 {
        const UNKNOWN   = 0x01;
        const RESPAWNS  = 0x02;
    }
}

impl FromRecordBytes for ContainerData {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(tuple((ContainerFlags::parse, le_f32)), |(flags, weight)| {
            Self { flags, weight }
        })(input)
    }
}

impl FromRecordBytes for ContainerFlags {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}
