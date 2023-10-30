use crate::error::Error;
use binrw::binrw;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::{Read, Seek};

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormID(pub u32);

impl fmt::Display for FormID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FormID({:#010X})", self.0)
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LocalizedString {
    Localized(u32),
    ZString(String),
}

impl fmt::Display for LocalizedString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LocalizedString::Localized(l) => write!(f, "LocalizedString::Localized({:?})", l),
            LocalizedString::ZString(z) => write!(f, "{}", z),
        }
    }
}

pub fn check_done_reading<T: Read + Seek>(reader: &mut T) -> Result<(), Error> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    if buf.is_empty() {
        Ok(())
    } else {
        Err(Error::ExtraBytes(buf))
    }
}

#[binrw]
#[brw(little)]
#[derive(Clone, Serialize, Deserialize)]
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
#[derive(Clone, Serialize, Deserialize)]
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
