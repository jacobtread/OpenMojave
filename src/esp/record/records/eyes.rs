use super::prelude::*;

/// Eyes
#[derive(Debug)]
pub struct EYES {
    pub editor_id: EditorId,
    pub name: String,
    pub texture: Option<String>,
    pub flags: EyeFlags,
}

impl Record for EYES {
    const TYPE: RecordType = RecordType::new(b"EYES");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id = parser.parse::<EditorId>(EDID)?;
        let name = parser.parse::<String>(FULL)?;
        let texture = parser.try_parse::<String>(ICON)?;
        let flags = parser.parse::<EyeFlags>(DATA)?;
        Ok(Self {
            editor_id,
            name,
            texture,
            flags,
        })
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct EyeFlags: u8 {
        const PLAYABLE   = 0x01;
        const NOT_MALE   = 0x02;
        const NOT_FEMALE = 0x04;
    }
}

impl FromRecordBytes for EyeFlags {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}
