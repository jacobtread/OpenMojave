use super::prelude::*;

/// Voice type
#[derive(Debug)]
pub struct VTYP {
    pub editor_id: EditorId,
    pub flags: Option<VoiceTypeFlags>,
}

impl Record for VTYP {
    const TYPE: RecordType = RecordType::new(b"VTYP");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let flags: Option<VoiceTypeFlags> = parser.try_parse(DNAM)?;
        Ok(Self { editor_id, flags })
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct VoiceTypeFlags: u8 {
        const ALLOW_DEFAULT_DIALOG = 0x01;
        const FEMALE               = 0x02;
    }
}

impl FromRecordBytes for VoiceTypeFlags {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}
