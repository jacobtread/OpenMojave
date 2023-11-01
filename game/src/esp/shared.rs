use super::error::EspError;
use super::record::FromSubRecord;
use binrw::binrw;
use nom::combinator::map;
use nom::number::complete::le_u32;
use nom::IResult;
use std::fmt;
use std::io::Read;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct EditorId(pub String);

impl Deref for EditorId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromSubRecord for EditorId {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(String::parse, Self)(input)
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct FormId(pub u32);

impl FromSubRecord for FormId {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(le_u32, FormId)(input)
    }
}

impl FormId {
    pub fn parse(input: &[u8]) -> IResult<&[u8], FormId> {
        map(le_u32, FormId)(input)
    }
}

#[derive(Debug, Clone)]
pub enum LocalizedString {
    Localized(u32),
    ZString(String),
}

/// Requires that the provided reader has no remaining bytes
/// left or else a [`EspError::ExtraBytes`] error is returned
pub fn require_complete<R>(reader: &mut R) -> Result<(), EspError>
where
    R: Read,
{
    let mut remaining = Vec::new();
    reader.read_to_end(&mut remaining)?;

    if remaining.is_empty() {
        Ok(())
    } else {
        Err(EspError::ExtraBytes(remaining))
    }
}

#[binrw]
#[brw(little)]
#[derive(Clone)]
pub struct WString {
    pub size: u16,
    #[br(count = size)]
    pub data: Vec<u8>,
}

impl fmt::Debug for WString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", String::from_utf8_lossy(&self.data))
    }
}

impl fmt::Display for WString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.data[..]))
    }
}

#[binrw]
#[brw(little)]
#[derive(Clone)]
pub struct WString32 {
    pub size: u32,
    #[br(count = size)]
    pub data: Vec<u8>,
}

impl fmt::Debug for WString32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", String::from_utf8_lossy(&self.data))
    }
}

impl fmt::Display for WString32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.data[..]))
    }
}
