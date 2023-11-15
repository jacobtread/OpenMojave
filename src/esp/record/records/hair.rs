use super::prelude::*;
use crate::esp::record::sub::model::ModelData;

/// Hair
#[derive(Debug)]
pub struct HAIR {
    pub editor_id: EditorId,
    pub name: String,
    pub model_data: ModelData,
    pub texture: String,
    pub flags: HairFlags,
}

impl Record for HAIR {
    const TYPE: RecordType = RecordType::new(b"HAIR");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id = parser.parse::<EditorId>(EDID)?;
        let name = parser.parse::<String>(FULL)?;
        let model_data = ModelData::require(parser)?;
        let texture = parser.parse::<String>(ICON)?;
        let flags = parser.parse::<HairFlags>(DATA)?;
        Ok(Self {
            editor_id,
            name,
            model_data,
            texture,
            flags,
        })
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct HairFlags: u8 {
        const PLAYABLE   = 0x01;
        const NOT_MALE   = 0x02;
        const NOT_FEMALE = 0x04;
        const FIXED      = 0x08;
    }
}

impl FromRecordBytes for HairFlags {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}
