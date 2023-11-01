#![allow(clippy::upper_case_acronyms)]

use nom::{
    bytes::complete::{tag, take_while},
    combinator::{map, rest},
    sequence::terminated,
    IResult,
};

pub mod achr;
pub mod collection;
pub mod gmst;
pub mod record;
pub mod sub;
pub mod tes4;

pub fn parse_cstring(input: &[u8]) -> IResult<&[u8], String> {
    // TODO: Possibly use CString instead
    map(
        terminated(
            // While non null
            take_while(|byte: u8| byte != 0),
            // Null terminator tag
            tag("\0"),
        ),
        |bytes| String::from_utf8_lossy(bytes).to_string(),
    )(input)
}
