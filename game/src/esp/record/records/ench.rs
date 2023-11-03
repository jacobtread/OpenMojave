use nom::{
    bytes::complete::take,
    combinator::map,
    number::complete::{le_u32, u8},
    sequence::tuple,
};

use crate::esp::{
    record::{
        sub::{effect::Effect, EDID, ENIT, FULL},
        FromRecordBytes, Record, RecordType,
    },
    shared::EditorId,
};

/// Object effect / Enchantment
#[derive(Debug)]
pub struct ENCH {
    pub editor_id: EditorId,
    pub name: Option<String>,
    pub effect_data: EffectData,
    pub effects: Vec<Effect>,
}

impl Record for ENCH {
    const TYPE: RecordType = RecordType::from_value(b"ENCH");

    fn parse<'b>(
        parser: &mut crate::esp::record::RecordParser<'_, 'b>,
    ) -> Result<Self, crate::esp::record::RecordParseError<'b>> {
        let editor_id = parser.parse::<EditorId>(EDID)?;
        let name = parser.try_parse::<String>(FULL)?;
        let effect_data = parser.parse::<EffectData>(ENIT)?;
        let effects = parser.parse_collection::<Effect>()?;
        if effects.is_empty() {
            return Err(crate::esp::record::RecordParseError::Custom(
                "Missing enchantment effect".to_string(),
            ));
        }

        Ok(Self {
            editor_id,
            name,
            effect_data,
            effects,
        })
    }
}

#[derive(Debug)]
pub struct EffectData {
    pub ty: u32,
    pub flags: u8,
}

impl FromRecordBytes for EffectData {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(
            tuple((
                le_u32,
                // Unused
                le_u32,
                // Unused
                le_u32,
                u8,
                // Unused
                take(3usize),
            )),
            |(ty, _, _, flags, _)| Self { ty, flags },
        )(input)
    }
}
