use super::prelude::*;

/// Global variable
#[derive(Debug)]
pub struct GLOB {
    pub editor_id: EditorId,
    pub ty: GlobalType,
    // TODO: Should this be type specific?
    pub value: f32,
}

impl Record for GLOB {
    const TYPE: RecordType = RecordType::new(b"GLOB");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let ty: GlobalType = parser.parse(FNAM)?;
        let value: f32 = parser.parse(FLTV)?;
        Ok(Self {
            editor_id,
            ty,
            value,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum GlobalType {
    Short = b's',
    Long = b'l',
    Float = b'f',
}

impl FromRecordBytes for GlobalType {
    #[inline]
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        enum_value(input)
    }
}
