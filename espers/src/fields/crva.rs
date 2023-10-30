use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead};
use serde_derive::{Deserialize, Serialize};
#[binrw]
#[brw(little, magic = b"CRVA")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CRVA {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CrimeGold {
    pub arrest: u8,
    pub attack_on_sight: u8,
    pub murder: u16,
    pub assault: u16,
    pub trespass: u16,
    pub pickpocket: u16,
    pub unused: u16,
    #[br(try)]
    pub steal: Option<f32>,
    #[br(try)]
    pub escape: Option<u16>,
    #[br(try)]
    pub werewolf: Option<u16>,
}

impl TryInto<CrimeGold> for CRVA {
    type Error = Error;

    fn try_into(self) -> Result<CrimeGold, Error> {
        Ok(CrimeGold::read_le(&mut Cursor::new(&self.data))?)
    }
}
