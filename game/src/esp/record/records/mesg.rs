use super::{
    micn::MICN,
    prelude::{condition::CTDA, *},
};

/// Message
#[derive(Debug)]
pub struct MESG {
    pub editor_id: EditorId,
    pub description: String,
    pub name: Option<String>,
    pub icon: NTypedFormId<MICN>,
    pub flags: MessageFlags,
    pub display_time: Option<u32>,
    pub buttons: Vec<MessageMenuButton>,
}

impl Record for MESG {
    const TYPE: RecordType = RecordType::new(b"MESG");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let description: String = parser.parse(DESC)?;
        let name: Option<String> = parser.try_parse(FULL)?;
        let icon: NTypedFormId<MICN> = parser.parse(INAM)?;

        parser.skip_type(NAM1);
        parser.skip_type(NAM2);
        parser.skip_type(NAM3);
        parser.skip_type(NAM4);
        parser.skip_type(NAM5);
        parser.skip_type(NAM6);
        parser.skip_type(NAM7);
        parser.skip_type(NAM8);
        parser.skip_type(NAM9);

        let flags: MessageFlags = parser.parse(DNAM)?;
        let display_time: Option<u32> = parser.try_parse(TNAM)?;
        let buttons: Vec<MessageMenuButton> = parser.parse_collection()?;

        Ok(Self {
            editor_id,
            description,
            name,
            icon,
            flags,
            display_time,
            buttons,
        })
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct MessageFlags: u32 {
        const MESSAGE_BOX  = 0x00000001;
        const AUTO_DISPLAY = 0x00000002;
    }
}

#[derive(Debug)]
pub struct MessageMenuButton {
    pub text: Option<String>,
    pub conditions: Vec<CTDA>,
}

impl FromRecordBytes for MessageFlags {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(le_u32, Self::from_bits_retain)(input)
    }
}

impl RecordCollection for MessageMenuButton {
    fn parse_next<'b>(
        parser: &mut RecordParser<'_, 'b>,
    ) -> Result<Option<Self>, RecordParseError<'b>> {
        let text: Option<String> = parser.try_parse(ITXT)?;
        let conditions: Vec<CTDA> = parser.try_parse_many(CTDA)?;

        Ok(if text.is_some() || !conditions.is_empty() {
            Some(Self { text, conditions })
        } else {
            None
        })
    }
}
