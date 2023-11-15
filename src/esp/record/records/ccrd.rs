use super::{
    prelude::{model::ModelData, object_bounds::ObjectBounds, *},
    scpt::SCPT,
    soun::SOUN,
};

/// Caravan Card
#[derive(Debug)]
pub struct CCRD {
    pub editor_id: EditorId,
    pub object_bounds: ObjectBounds,
    pub name: Option<String>,
    pub model_data: Option<ModelData>,
    pub large_icon_file_name: Option<String>,
    pub small_icon_file_name: Option<String>,
    pub script: Option<TypedFormId<SCPT>>,
    pub sound_pick_up: Option<TypedFormId<SOUN>>,
    pub sound_drop: Option<TypedFormId<SOUN>>,
    pub high_res_image_face: Option<String>,
    pub high_res_image_back: Option<String>,
    pub card_suit: Option<CardSuit>,
    pub card_value: Option<CardValue>,
    pub value: Option<u32>,
}

impl Record for CCRD {
    const TYPE: RecordType = CCRD;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let object_bounds: ObjectBounds = parser.parse(OBND)?;
        let name: Option<String> = parser.try_parse(FULL)?;
        let model_data: Option<ModelData> = ModelData::parse_first(parser)?;
        let large_icon_file_name: Option<String> = parser.try_parse(ICON)?;
        let small_icon_file_name: Option<String> = parser.try_parse(MICO)?;
        let script: Option<TypedFormId<SCPT>> = parser.try_parse(SCRI)?;
        let sound_pick_up: Option<TypedFormId<SOUN>> = parser.try_parse(YNAM)?;
        let sound_drop: Option<TypedFormId<SOUN>> = parser.try_parse(ZNAM)?;
        let high_res_image_face: Option<String> = parser.try_parse(TX00)?;
        let high_res_image_back: Option<String> = parser.try_parse(TX01)?;
        let card_suit: Option<CardSuit> = parser.try_parse(INTV)?;
        let card_value: Option<CardValue> = parser.try_parse(INTV)?;
        let value: Option<u32> = parser.try_parse(DATA)?;

        Ok(Self {
            editor_id,
            object_bounds,
            name,
            model_data,
            large_icon_file_name,
            small_icon_file_name,
            script,
            sound_pick_up,
            sound_drop,
            high_res_image_face,
            high_res_image_back,
            card_suit,
            card_value,
            value,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum CardSuit {
    U1 = 0,
    Hearts = 1,
    Spades = 2,
    Diamonds = 3,
    Clubs = 4,
    Joker = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum CardValue {
    U1 = 0,
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    U2 = 11,
    Jack = 12,
    Queen = 13,
    King = 14,
    Joker = 15,
}

impl FromRecordBytes for CardSuit {
    #[inline]
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        enum_value(input)
    }
}

impl FromRecordBytes for CardValue {
    #[inline]
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        enum_value(input)
    }
}
