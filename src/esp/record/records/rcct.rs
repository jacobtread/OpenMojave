use super::prelude::*;

/// Recipe Category
#[derive(Debug)]
pub struct RCCT {
    pub editor_id: EditorId,
    pub name: Option<String>,
    pub flags: Option<RecipeCategoryFlags>,
}

impl Record for RCCT {
    const TYPE: RecordType = RecordType::new(b"RCCT");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let name: Option<String> = parser.try_parse(FULL)?;
        let flags: Option<RecipeCategoryFlags> = parser.try_parse(DATA)?;
        Ok(Self {
            editor_id,
            name,
            flags,
        })
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct RecipeCategoryFlags: u8 {
        const SUBCATEGORY = 0x01;
        const U1          = 0x02;
        const U2          = 0x04;
        const U3          = 0x08;
        const U4          = 0x10;
        const U5          = 0x20;
        const U6          = 0x40;
        const U7          = 0x80;

    }
}

impl FromRecordBytes for RecipeCategoryFlags {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(u8, Self::from_bits_retain)(input)
    }
}
