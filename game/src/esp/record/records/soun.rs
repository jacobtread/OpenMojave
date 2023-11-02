use bitflags::bitflags;
use nom::{
    combinator::map,
    number::complete::{i8, le_i16, le_i32, le_u32, u8},
    sequence::tuple,
    IResult,
};

use crate::esp::{
    record::{
        sub::{ANAM, EDID, FNAM, GNAM, HNAM, OBND, RNAM, SNDD, SNDX},
        FromRecordBytes, Record, RecordParseError, RecordParser, RecordType,
    },
    shared::EditorId,
};

use super::txst::OBND;

/// Sound
#[derive(Debug)]
pub struct SOUN {
    pub editor_id: EditorId,
    pub object_bounds: OBND,
    pub file_name: Option<String>,
    pub random_chance: Option<u8>,
    pub data: SoundData,
    pub attenu_points: Option<AttenuPoints>,
    pub reverb_attenu_control: Option<i16>,
    pub priority: Option<i32>,
}

impl Record for SOUN {
    const TYPE: RecordType = RecordType::from_value(b"SOUN");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id = parser.parse::<EditorId>(EDID)?;
        let object_bounds = parser.parse::<OBND>(OBND)?;

        let file_name = parser.try_parse::<String>(FNAM)?;
        let random_chance = parser.try_parse::<u8>(RNAM)?;

        let data: SoundData = if let Some(value) = parser.try_parse::<SNDD>(SNDD)? {
            SoundData::SNDD(value)
        } else if let Some(value) = parser.try_parse::<SNDX>(SNDX)? {
            SoundData::SNDX(value)
        } else {
            return Err(RecordParseError::Custom("Sound missing data".to_string()));
        };

        let attenu_points = parser.try_parse::<AttenuPoints>(ANAM)?;
        let reverb_attenu_control = parser.try_parse::<i16>(GNAM)?;
        let priority = parser.try_parse::<i32>(HNAM)?;

        Ok(SOUN {
            editor_id,
            object_bounds,
            file_name,
            random_chance,
            data,
            attenu_points,
            reverb_attenu_control,
            priority,
        })
    }
}

#[derive(Debug)]
pub struct AttenuPoints([i16; 5]);

impl FromRecordBytes for AttenuPoints {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(tuple((le_i16, le_i16, le_i16, le_i16, le_i16)), |values| {
            Self(values.into())
        })(input)
    }
}

#[derive(Debug)]
pub enum SoundData {
    SNDD(SNDD),
    SNDX(SNDX),
}

// Dyanmic data
#[derive(Debug)]
pub struct SNDD {
    pub sndx: SNDX,
    pub attenu_points: AttenuPoints,
    pub reverb_attenu_control: i16,
    pub priority: i32,
    pub x: i32,
    pub y: i32,
}

impl FromRecordBytes for SNDD {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((
                SNDX::parse,
                AttenuPoints::parse,
                le_i16,
                le_i32,
                le_i32,
                le_i32,
            )),
            |(sndx, attenu_points, reverb_attenu_control, priority, x, y)| Self {
                sndx,
                attenu_points,
                reverb_attenu_control,
                priority,
                x,
                y,
            },
        )(input)
    }
}

// Static data
#[derive(Debug)]
pub struct SNDX {
    pub min_attenu_distance: u8,
    pub max_attenu_distance: u8,
    pub freq_adjustment_percent: i8,
    pub flags: Flags,
    pub static_attenu_cd_b: i16,
    pub stop_time: u8,
    pub start_time: u8,
}

impl FromRecordBytes for SNDX {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((u8, u8, i8, Flags::parse, le_i16, u8, u8)),
            |(
                min_attenu_distance,
                max_attenu_distance,
                freq_adjustment_percent,
                flags,
                static_attenu_cd_b,
                stop_time,
                start_time,
            )| Self {
                min_attenu_distance,
                max_attenu_distance,
                freq_adjustment_percent,
                flags,
                static_attenu_cd_b,
                stop_time,
                start_time,
            },
        )(input)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct Flags: u32 {
        const RANDOM_FREQUENCY_SHIFT = 0x00000001;
        const PLAY_AT_RANDOM = 0x00000002;
        const ENVIRONMENT_IGNORED = 0x00000004;
        const RANDOM_LOCATION = 0x00000008;
        const LOOP = 0x00000010;
        const MENU_SOUND = 0x00000020;
        const F2D = 0x00000040; // 2D
        const F360_LFE = 0x00000080; // 360 LFE
        const DIALOGUE_SOUND = 0x00000100;
        const ENVELOPE_FAST = 0x00000200;
        const ENVELOPE_SLOW = 0x00000400;
        const RADIUS_2D = 0x00000800;
        const MUTE_WHEN_SUBMERGED = 0x00001000;
        const START_AT_RANDOM_POSITION = 0x00002000;
    }
}

impl FromRecordBytes for Flags {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(le_u32, Self::from_bits_retain)(input)
    }
}
