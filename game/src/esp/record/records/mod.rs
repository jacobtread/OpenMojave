pub mod prelude {
    pub use crate::esp::record::{
        FromRecordBytes, Record, RecordCollection, RecordParseError, RecordParser, RecordType,
    };

    pub use nom::{combinator::map, number::complete::*, sequence::tuple, IResult};
}

pub mod acti;
pub mod armo;
pub mod aspc;
pub mod book;
pub mod clas;
pub mod cont;
pub mod door;
pub mod ench;
pub mod eyes;
pub mod fact;
pub mod furn;
pub mod gmst;
pub mod gras;
pub mod hair;
pub mod hdpt;
pub mod ingr;
pub mod ligh;
pub mod ltex;
pub mod mgef;
pub mod micn;
pub mod misc;
pub mod mstt;
pub mod pwat;
pub mod race;
pub mod scol;
pub mod scpt;
pub mod soun;
pub mod spel;
pub mod stat;
pub mod tact;
pub mod term;
pub mod tes4;
pub mod tree;
pub mod txst;
pub mod weap;
