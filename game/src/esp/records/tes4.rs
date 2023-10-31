use nom::{
    bytes::complete::*, combinator::*, multi::many0, number::complete::*, sequence::tuple, IResult,
};

#[derive(Debug)]
pub struct Header {
    hedr: HEDR,
}

#[derive(Debug)]
pub struct HEDR {
    pub version: f32,
    pub records_and_groups_count: u32,
    pub next_available_object_id: u32,
}

fn parse_hedr(input: &[u8]) -> IResult<&[u8], HEDR> {
    map(
        tuple((tag(b"HEDR"), le_u32, le_f32, le_u32, le_u32)),
        |(_, _, version, records_and_groups_count, next_available_object_id)| HEDR {
            version,
            records_and_groups_count,
            next_available_object_id,
        },
    )(input)
}

pub fn parse_tex4(input: &[u8]) -> IResult<&[u8], Header> {
    let (input, hedr) = parse_hedr(input)?;
    Ok((input, Header { hedr }))
}

// /// Main plugin header
// #[binrw]
// #[derive(Debug, Clone)]
// #[brw(little, magic = b"TES4")]
// pub struct TES4 {
//     pub header: RecordHeader,

//     #[br(count = header.size)]
//     pub data: Vec<u8>,
// }

// #[binrw]
// #[brw(little, magic = b"HEDR")]
// #[derive(Debug, Clone)]
// pub struct HEDR {
//     pub size: u16,
//     //
//     pub version: f32,
//     pub records_and_groups_count: u32,
//     pub next_available_object_id: u32,
// }

// /// Parsed main plugin header
// #[derive(Debug, Clone)]
// pub struct Header {
//     /// Raw underlying record header
//     pub header: RecordHeader,
//     // Parsed option
//     pub hedr: Option<HEDR>,
//     pub author: Option<String>,
//     pub description: Option<String>,
//     pub dependencies: Option<String>,
//     pub screenshot: Option<String>,
// }
