use nom::{
    combinator::map,
    number::complete::{le_f32, le_i32, le_u32},
    sequence::tuple,
};

use crate::esp::{
    record::{FromRecordBytes, RecordCollection},
    shared::FormId,
};

use super::{CNTO, COED};

#[derive(Debug)]
pub struct Item {
    pub item: Option<CNTO>,
    pub extra_data: Option<COED>,
}

impl RecordCollection for Item {
    fn parse_next<'b>(
        parser: &mut crate::esp::record::RecordParser<'_, 'b>,
    ) -> Result<Option<Self>, crate::esp::record::RecordParseError<'b>> {
        let item: Option<CNTO> = parser.try_parse(CNTO)?;
        let extra_data: Option<COED> = parser.try_parse(COED)?;

        if item.is_some() || extra_data.is_some() {
            Ok(Some(Self { item, extra_data }))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug)]
pub struct CNTO {
    /// FormID of a ARMO, AMMO, MISC, WEAP, BOOK, LVLI, KEYM, ALCH, NOTE, IMOD, CMNY, CCRD, LIGH, CHIP, MSTT or STAT record.
    pub item: FormId,
    pub count: i32,
}

impl FromRecordBytes for CNTO {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(tuple((FormId::parse, le_i32)), |(item, count)| Self {
            item,
            count,
        })(input)
    }
}

#[derive(Debug)]
pub struct COED {
    /// FormID of a NPC_ or FACT record, or null.
    pub owner: FormId,
    /// FormID of a GLOB record, an integer representing the required rank, or null.
    pub global_variable: u32,

    pub item_condition: f32,
}

impl FromRecordBytes for COED {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(
            tuple((FormId::parse, le_u32, le_f32)),
            |(owner, global_variable, item_condition)| Self {
                owner,
                global_variable,
                item_condition,
            },
        )(input)
    }
}
