use super::{
    prelude::{model::ModelData, object_bounds::ObjectBounds, *},
    qust::QUST,
    soun::SOUN,
};
use crate::esp::record::RawBytes;

/// Note
#[derive(Debug)]
pub struct NOTE {
    pub editor_id: EditorId,
    pub object_bounds: ObjectBounds,
    pub name: String,
    pub model_data: Option<ModelData>,
    pub large_icon_file_name: Option<String>,
    pub small_icon_file_name: Option<String>,
    pub sound_pick_up: Option<TypedFormId<SOUN>>,
    pub sound_drop: Option<TypedFormId<SOUN>>,
    pub ty: Option<NoteType>,
    pub quests: Vec<TypedFormId<QUST>>,
    pub texture: Option<String>,
    pub text_topic: Option<NoteTopic>,
    /// FormID of a SOUN, NPC_ or CREA record.
    pub actor: Option<FormId>,
}

impl Record for NOTE {
    const TYPE: RecordType = RecordType::new(b"NOTE");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let object_bounds: ObjectBounds = parser.parse(OBND)?;
        let name: String = parser.parse(FULL)?;
        let model_data: Option<ModelData> = ModelData::parse_first(parser)?;
        let large_icon_file_name: Option<String> = parser.try_parse(ICON)?;
        let small_icon_file_name: Option<String> = parser.try_parse(MICO)?;
        let sound_pick_up: Option<TypedFormId<SOUN>> = parser.try_parse(YNAM)?;
        let sound_drop: Option<TypedFormId<SOUN>> = parser.try_parse(ZNAM)?;
        let ty: Option<NoteType> = parser.try_parse(DATA)?;
        let quests: Vec<TypedFormId<QUST>> = parser.try_parse_many(ONAM)?;
        let texture: Option<String> = parser.try_parse(XNAM)?;
        let text_topic: Option<NoteTopic> = parser.try_parse(TNAM)?;
        let actor: Option<FormId> = parser.try_parse(SNAM)?;

        Ok(Self {
            editor_id,
            object_bounds,
            name,
            model_data,
            large_icon_file_name,
            small_icon_file_name,
            sound_pick_up,
            sound_drop,
            ty,
            quests,
            texture,
            text_topic,
            actor,
        })
    }
}

/// Could be either a String or TypedFormId<DIAL>
#[derive(Debug)]
pub struct NoteTopic(RawBytes);

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum NoteType {
    Sound = 0,
    Text = 1,
    Image = 2,
    Voice = 3,
}

impl FromRecordBytes for NoteTopic {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(RawBytes::parse, Self)(input)
    }
}

impl FromRecordBytes for NoteType {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        enum_value(input)
    }
}
