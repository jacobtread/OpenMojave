use bitflags::bitflags;
use nom::{
    bytes::complete::take,
    character::complete::u8,
    combinator::map,
    number::complete::{le_f32, le_u16},
    sequence::tuple,
    IResult,
};

use crate::esp::{
    record::{
        sub::{
            object_bounds::ObjectBounds, DNAM, DODT, EDID, OBND, TX00, TX01, TX02, TX03, TX04, TX05,
        },
        FromRecordBytes, Record, RecordParseError, RecordParser, RecordType,
    },
    shared::{EditorId, RGBA},
};

#[derive(Debug)]
pub struct TXST {
    pub editor_id: EditorId,
    pub object_bounds: ObjectBounds,
    pub tx00: Option<String>,
    pub tx01: Option<String>,
    pub tx02: Option<String>,
    pub tx03: Option<String>,
    pub tx04: Option<String>,
    pub tx05: Option<String>,
    pub decal_data: Option<DODT>,
    pub flags: TXSTFlags,
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct TXSTFlags: u16 {
        const NO_SPECULAR_MAP = 0x001;
    }
}

impl FromRecordBytes for TXSTFlags {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(le_u16, TXSTFlags::from_bits_retain)(input)
    }
}

impl Record for TXST {
    const TYPE: RecordType = RecordType::new(b"TXST");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id = parser.parse::<EditorId>(EDID)?;
        let object_bounds = parser.parse::<ObjectBounds>(OBND)?;
        let tx00 = parser.try_parse::<String>(TX00)?;
        let tx01 = parser.try_parse::<String>(TX01)?;
        let tx02 = parser.try_parse::<String>(TX02)?;
        let tx03 = parser.try_parse::<String>(TX03)?;
        let tx04 = parser.try_parse::<String>(TX04)?;
        let tx05 = parser.try_parse::<String>(TX05)?;
        let decal_data = parser.try_parse::<DODT>(DODT)?;
        let flags = parser.parse::<TXSTFlags>(DNAM)?;

        Ok(Self {
            editor_id,
            object_bounds,
            tx00,
            tx01,
            tx02,
            tx03,
            tx04,
            tx05,
            decal_data,
            flags,
        })
    }
}

#[derive(Debug)]
pub struct DODT {
    pub min_width: f32,
    pub max_width: f32,
    pub min_height: f32,
    pub max_height: f32,
    pub depth: f32,
    pub shininess: f32,
    pub parallax_scale: f32,
    pub parallax_passes: u8,
    pub flags: DODTFlags,
    pub color: RGBA,
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct DODTFlags: u8 {
        const PARALAX = 0x01;
        const ALPHA_BLENDING = 0x02;
        const ALPHA_TESTINGT = 0x04;
    }
}

impl DODTFlags {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(u8, DODTFlags::from_bits_retain)(input)
    }
}

impl FromRecordBytes for DODT {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(
            tuple((
                le_f32,
                le_f32,
                le_f32,
                le_f32,
                le_f32,
                le_f32,
                le_f32,
                u8,
                DODTFlags::parse,
                // Unused bytes
                take(2usize),
                RGBA::parse,
            )),
            |(
                min_width,
                max_width,
                min_height,
                max_height,
                depth,
                shininess,
                parallax_scale,
                parallax_passes,
                flags,
                _,
                color,
            )| Self {
                min_width,
                max_width,
                min_height,
                max_height,
                depth,
                shininess,
                parallax_scale,
                parallax_passes,
                flags,
                color,
            },
        )(input)
    }
}
