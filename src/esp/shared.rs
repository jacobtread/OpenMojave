use super::error::EspError;
use super::record::FromRecordBytes;
use binrw::binrw;
use nom::bytes::complete;
use nom::combinator::{complete, map};
use nom::multi::{length_data, length_value};
use nom::number::complete::{be_u16, le_u16, le_u32, u8};
use nom::sequence::tuple;
use nom::IResult;
use std::any::type_name;
use std::fmt::{self, Debug};
use std::io::Read;
use std::marker::PhantomData;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct EditorId(pub String);

impl Deref for EditorId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRecordBytes for EditorId {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(String::parse, Self)(input)
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
pub struct RGBA {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    // Unused?
    pub alpha: u8,
}

impl FromRecordBytes for RGBA {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(tuple((u8, u8, u8, u8)), |(red, green, blue, alpha)| Self {
            red,
            green,
            blue,
            alpha,
        })(input)
    }
}

/// Type alias for a typed form id that might be null
pub type NTypedFormId<T> = TypedFormId<T>;

/// Represents a FormId of a specific type
pub struct TypedFormId<T> {
    pub id: FormId,
    pub _marker: PhantomData<T>,
}

impl<T> TypedFormId<T> {
    pub fn is_null(&self) -> bool {
        self.id.is_null()
    }
}

impl<T> Debug for TypedFormId<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TypedFormId<{}>({:#06x})", type_name::<T>(), self.id.0)
    }
}

impl<T> FromRecordBytes for TypedFormId<T> {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(FormId::parse, FormId::into_typed)(input)
    }
}

#[derive(Debug, Clone)]
pub struct FormId(pub u32);

impl FromRecordBytes for FormId {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(le_u32, FormId)(input)
    }
}

impl FormId {
    pub const NULL_RAW: u32 = 0;
    pub const NULL: FormId = FormId(Self::NULL_RAW);

    pub fn is_null(&self) -> bool {
        self.0 == Self::NULL_RAW
    }

    pub fn into_typed<T>(self) -> TypedFormId<T> {
        TypedFormId {
            id: self,
            _marker: PhantomData,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LocalizedString {
    Localized(u32),
    ZString(String),
}

/// String where the length is provided by a leading u16 value
pub struct String16(pub String);

impl FromRecordBytes for String16 {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(length_data(le_u16), |bytes| {
            Self(String::from_utf8_lossy(bytes).to_string())
        })(input)
    }
}

/// String where the length is provided by a leading u32 value
pub struct String32(pub String);

impl FromRecordBytes for String32 {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(length_data(le_u32), |bytes| {
            Self(String::from_utf8_lossy(bytes).to_string())
        })(input)
    }
}
