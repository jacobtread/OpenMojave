use super::prelude::*;
use crate::esp::record::sub::effect::Effect;

/// Object Effect
#[derive(Debug)]
pub struct ENCH {
    pub editor_id: EditorId,
    pub name: Option<String>,
    pub effect_data: EffectData,
    pub effects: Vec<Effect>,
}

impl Record for ENCH {
    const TYPE: RecordType = RecordType::new(b"ENCH");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let name: Option<String> = parser.try_parse(FULL)?;
        let effect_data: EffectData = parser.parse(ENIT)?;
        let effects: Vec<Effect> = parser.parse_collection()?;
        if effects.is_empty() {
            return Err(RecordParseError::Custom(
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
