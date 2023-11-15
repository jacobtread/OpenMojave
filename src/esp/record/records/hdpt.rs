use super::prelude::*;
use crate::esp::record::sub::model::ModelData;

/// Head part
#[derive(Debug)]
pub struct HDPT {
    pub editor_id: EditorId,
    pub name: String,
    pub model_data: Option<ModelData>,
    pub flags: HeadPartFlags,
    pub extra_parts: Vec<FormId>,
}

impl Record for HDPT {
    const TYPE: RecordType = RecordType::new(b"HDPT");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id = parser.parse::<EditorId>(EDID)?;
        let name = parser.parse::<String>(FULL)?;
        // TODO: Not sure if this field is optional documentation unclear
        let model_data = ModelData::parse_first(parser)?;
        let flags = parser.parse::<HeadPartFlags>(DATA)?;
        let extra_parts = parser.parse::<Repeated<FormId>>(HNAM)?.into_inner();

        Ok(Self {
            editor_id,
            name,
            model_data,
            flags,
            extra_parts,
        })
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct HeadPartFlags: u8 {
        const PLAYABLE = 0x01;
    }
}

impl FromRecordBytes for HeadPartFlags {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}
