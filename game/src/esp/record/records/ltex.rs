use super::{gras::GRAS, prelude::*, txst::TXST};

/// Landscape texture
#[derive(Debug)]
pub struct LTEX {
    pub editor_id: EditorId,
    pub large_icon_file_name: String,
    pub small_icon_file_name: String,
    pub texture: TypedFormId<TXST>,
    pub havok_data: HavokData,
    pub texture_specular_exponent: u8,
    pub grass: Vec<TypedFormId<GRAS>>,
}

impl Record for LTEX {
    const TYPE: RecordType = RecordType::new(b"LTEX");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let large_icon_file_name: String = parser.parse(ICON)?;
        let small_icon_file_name: String = parser.parse(MICO)?;
        let texture: TypedFormId<TXST> = parser.parse(TNAM)?;
        let havok_data: HavokData = parser.parse(HNAM)?;
        let texture_specular_exponent: u8 = parser.parse::<u8>(SNAM)?;
        let grass: Vec<TypedFormId<GRAS>> = parser.try_parse_many(GNAM)?;

        Ok(Self {
            editor_id,
            large_icon_file_name,
            small_icon_file_name,
            texture,
            havok_data,
            texture_specular_exponent,
            grass,
        })
    }
}

#[derive(Debug)]
pub struct HavokData {
    pub ty: MaterialType,
    pub friction: u8,
    pub restitution: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum MaterialType {
    Stone = 0,
    Cloth = 1,
    Dirt = 2,
    Glass = 3,
    Grass = 4,
    Metal = 5,
    Organic = 6,
    Skin = 7,
    Water = 8,
    Wood = 9,
    HeavyStone = 10,
    HeavyMetal = 11,
    HeavyWood = 12,
    Chain = 13,
    Snow = 14,
    Elevator = 15,
    HollowMetal = 16,
    SheetMetal = 17,
    Sand = 18,
    BrokenConcrete = 19,
    VehicleBody = 20,
    VehiclePartSolid = 21,
    VehiclePartHollow = 22,
    Barrel = 23,
    Bottle = 24,
    SodaCan = 25,
    Pistol = 26,
    Rifle = 27,
    ShoppingCart = 28,
    Lunchbox = 29,
    BabyRattle = 30,
    RubberBall = 31,
}

impl FromRecordBytes for HavokData {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(
            tuple((enum_value::<MaterialType>, u8, u8)),
            |(ty, friction, restitution)| Self {
                ty,
                friction,
                restitution,
            },
        )(input)
    }
}
