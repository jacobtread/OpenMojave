use super::{
    prelude::{condition::CTDA, skill::Skill, *},
    rcct::RCCT,
};

/// Recipe
#[derive(Debug)]
pub struct RCPE {
    pub editor_id: EditorId,
    pub name: Option<String>,
    pub conditions: Vec<CTDA>,
    pub data: Option<RecipeData>,
    pub ingredients: Vec<RecipeIngredient>,
    pub output: Vec<RecipeOutput>,
}

impl Record for RCPE {
    const TYPE: RecordType = RecordType::new(b"RCPE");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let name: Option<String> = parser.try_parse(FULL)?;
        let conditions: Vec<CTDA> = parser.try_parse_many(CTDA)?;
        let data: Option<RecipeData> = parser.try_parse(DATA)?;
        let ingredients: Vec<RecipeIngredient> = parser.parse_collection()?;
        let output: Vec<RecipeOutput> = parser.parse_collection()?;

        Ok(Self {
            editor_id,
            name,
            conditions,
            data,
            ingredients,
            output,
        })
    }
}

#[derive(Debug)]
pub struct RecipeData {
    pub skill: Skill,
    pub level: u32,
    pub category: TypedFormId<RCCT>,
    pub sub_category: TypedFormId<RCCT>,
}

#[derive(Debug)]
pub struct RecipeIngredient {
    /// FormID of a ARMO, AMMO, MISC, WEAP, BOOK, KEYM, ALCH, NOTE, IMOD, CMNY, CCRD, CHIP or LIGH record.
    pub item: FormId,
    pub quantity: u32,
}

#[derive(Debug)]
pub struct RecipeOutput {
    /// FormID of a ARMO, AMMO, MISC, WEAP, BOOK, KEYM, ALCH, NOTE, IMOD, CMNY, CCRD, CHIP or LIGH record.
    pub item: FormId,
    pub quantity: u32,
}

impl FromRecordBytes for RecipeData {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((enum_value, le_u32, TypedFormId::parse, TypedFormId::parse)),
            |(skill, level, category, sub_category)| Self {
                skill,
                level,
                category,
                sub_category,
            },
        )(input)
    }
}

impl RecordCollection for RecipeIngredient {
    fn parse_next<'b>(
        parser: &mut RecordParser<'_, 'b>,
    ) -> Result<Option<Self>, RecordParseError<'b>> {
        let item: FormId = match parser.try_parse(RCIL)? {
            Some(value) => value,
            None => return Ok(None),
        };
        let quantity: u32 = parser.parse(RCQY)?;
        Ok(Some(Self { item, quantity }))
    }
}

impl RecordCollection for RecipeOutput {
    fn parse_next<'b>(
        parser: &mut RecordParser<'_, 'b>,
    ) -> Result<Option<Self>, RecordParseError<'b>> {
        let item: FormId = match parser.try_parse(RCOD)? {
            Some(value) => value,
            None => return Ok(None),
        };
        let quantity: u32 = parser.parse(RCQY)?;
        Ok(Some(Self { item, quantity }))
    }
}
