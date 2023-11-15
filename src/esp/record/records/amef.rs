use super::prelude::*;

/// Ammo Effect
#[derive(Debug)]
pub struct AMEF {
    pub editor_id: EditorId,
    pub name: Option<String>,
    pub data: Option<AmmoEffectData>,
}

impl Record for AMEF {
    const TYPE: RecordType = AMEF;

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let name: Option<String> = parser.try_parse(FULL)?;
        let data: Option<AmmoEffectData> = parser.try_parse(DATA)?;
        Ok(Self {
            editor_id,
            name,
            data,
        })
    }
}

#[derive(Debug)]
pub struct AmmoEffectData {
    pub ty: AmmoEffectType,
    pub operation: AmmoEffectOperation,
    pub value: f32,
}

impl FromRecordBytes for AmmoEffectData {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((enum_value, enum_value, le_f32)),
            |(ty, operation, value)| Self {
                ty,
                operation,
                value,
            },
        )(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum AmmoEffectType {
    Damage = 0,
    DR = 1,
    DT = 2,
    Spread = 3,
    WeaponCondition = 4,
    Fatigue = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum AmmoEffectOperation {
    Add = 0,
    Multiply = 1,
    Subtract = 2,
}
