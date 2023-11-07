use std::any::Any;

use prelude::RecordParser;

use super::{RawRecord, Record, RecordParseError, RecordType};

pub mod prelude {
    pub use crate::esp::{
        record::{
            enum_value, sub::*, take4, FromRecordBytes, Record, RecordCollection, RecordParseError,
            RecordParser, RecordType, Repeated,
        },
        shared::{EditorId, FormId, NTypedFormId, String16, String32, TypedFormId, RGBA},
    };
    pub use bitflags::bitflags;
    pub use fyrox::core::algebra::{Vector2, Vector3};
    pub use nom::{
        bytes::complete::take,
        combinator::{map, rest},
        number::complete::*,
        sequence::tuple,
        IResult,
    };
    pub use num_enum::TryFromPrimitive;
}

pub mod achr;
pub mod acre;
pub mod acti;
pub mod addn;
pub mod alch;
pub mod aloc;
pub mod amef;
pub mod ammo;
pub mod anio;
pub mod arma;
pub mod armo;
pub mod aspc;
pub mod avif;
pub mod book;
pub mod bptd;
pub mod cams;
pub mod ccrd;
pub mod cdck;
pub mod cell;
pub mod chal;
pub mod chip;
pub mod clas;
pub mod clmt;
pub mod cmny;
pub mod cobj;
pub mod cont;
pub mod cpth;
pub mod crea;
pub mod csno;
pub mod csty;
pub mod debr;
pub mod dehy;
pub mod dial;
pub mod dobj;
pub mod door;
pub mod eczn;
pub mod efsh;
pub mod ench;
pub mod expl;
pub mod eyes;
pub mod fact;
pub mod flst;
pub mod furn;
pub mod glob;
pub mod gmst;
pub mod gras;
pub mod hair;
pub mod hdpt;
pub mod hung;
pub mod idle;
pub mod idlm;
pub mod imad;
pub mod imgs;
pub mod imod;
pub mod info;
pub mod ingr;
pub mod ipct;
pub mod ipds;
pub mod keym;
pub mod land;
pub mod lgtm;
pub mod ligh;
pub mod lscr;
pub mod lsct;
pub mod ltex;
pub mod lvlc;
pub mod lvli;
pub mod lvln;
pub mod mesg;
pub mod mgef;
pub mod micn;
pub mod misc;
pub mod mset;
pub mod mstt;
pub mod musc;
pub mod navi;
pub mod navm;
pub mod note;
pub mod npc;
pub mod pack;
pub mod perk;
pub mod pgre;
pub mod pmis;
pub mod proj;
pub mod pwat;
pub mod qust;
pub mod race;
pub mod rads;
pub mod rcct;
pub mod rcpe;
pub mod refr;
pub mod regn;
pub mod repu;
pub mod rgdl;
pub mod scol;
pub mod scpt;
pub mod slpd;
pub mod soun;
pub mod spel;
pub mod stat;
pub mod tact;
pub mod term;
pub mod tes4;
pub mod tree;
pub mod txst;
pub mod vtyp;
pub mod watr;
pub mod weap;
pub mod wrld;
pub mod wthr;

pub enum ParsedRecord {
    ACHR(Box<achr::ACHR>),
    ACRE(Box<acre::ACRE>),
    ACTI(Box<acti::ACTI>),
    ADDN(Box<addn::ADDN>),
    ALCH(Box<alch::ALCH>),
    ALOC(Box<aloc::ALOC>),
    AMEF(Box<amef::AMEF>),
    AMMO(Box<ammo::AMMO>),
    ANIO(Box<anio::ANIO>),
    ARMO(Box<armo::ARMO>),
    ARMA(Box<arma::ARMA>),
    ASPC(Box<aspc::ASPC>),
    AVIF(Box<avif::AVIF>),
    BOOK(Box<book::BOOK>),
    BPTD(Box<bptd::BPTD>),
    CAMS(Box<cams::CAMS>),
    CCRD(Box<ccrd::CCRD>),
    CDCK(Box<cdck::CDCK>),
    CELL(Box<cell::CELL>),
    CHAL(Box<chal::CHAL>),
    CHIP(Box<chip::CHIP>),
    CLAS(Box<clas::CLAS>),
    CLMT(Box<clmt::CLMT>),
    CMNY(Box<cmny::CMNY>),
    COBJ(Box<cobj::COBJ>),
    CONT(Box<cont::CONT>),
    CPTH(Box<cpth::CPTH>),
    CREA(Box<crea::CREA>),
    CSNO(Box<csno::CSNO>),
    CSTY(Box<csty::CSTY>),
    DEBR(Box<debr::DEBR>),
    DEHY(Box<dehy::DEHY>),
    DIAL(Box<dial::DIAL>),
    DOBJ(Box<dobj::DOBJ>),
    DOOR(Box<door::DOOR>),
    ECZN(Box<eczn::ECZN>),
    EFSH(Box<efsh::EFSH>),
    ENCH(Box<ench::ENCH>),
    EXPL(Box<expl::EXPL>),
    EYES(Box<eyes::EYES>),
    FACT(Box<fact::FACT>),
    FLST(Box<flst::FLST>),
    FURN(Box<furn::FURN>),
    GLOB(Box<glob::GLOB>),
    GMST(Box<gmst::GMST>),
    GRAS(Box<gras::GRAS>),
    HAIR(Box<hair::HAIR>),
    HDPT(Box<hdpt::HDPT>),
    HUNG(Box<hung::HUNG>),
    IDLE(Box<idle::IDLE>),
    IDLM(Box<idlm::IDLM>),
    IMGS(Box<imgs::IMGS>),
    IMAD(Box<imad::IMAD>),
    IMOD(Box<imod::IMOD>),
    INFO(Box<info::INFO>),
    INGR(Box<ingr::INGR>),
    IPCT(Box<ipct::IPCT>),
    IPDS(Box<ipds::IPDS>),
    KEYM(Box<keym::KEYM>),
    LAND(Box<land::LAND>),
    LGTM(Box<lgtm::LGTM>),
    LIGH(Box<ligh::LIGH>),
    LSCR(Box<lscr::LSCR>),
    LSCT(Box<lsct::LSCT>),
    LTEX(Box<ltex::LTEX>),
    LVLC(Box<lvlc::LVLC>),
    LVLI(Box<lvli::LVLI>),
    LVLN(Box<lvln::LVLN>),
    MESG(Box<mesg::MESG>),
    MGEF(Box<mgef::MGEF>),
    MICN(Box<micn::MICN>),
    MISC(Box<misc::MISC>),
    NAVI(Box<navi::NAVI>),
    NAVM(Box<navm::NAVM>),
    NOTE(Box<note::NOTE>),
    NPC_(Box<npc::NPC_>),
    PACK(Box<pack::PACK>),
    PERK(Box<perk::PERK>),
    PGRE(Box<pgre::PGRE>),
    PMIS(Box<pmis::PMIS>),
    PROJ(Box<proj::PROJ>),
    PWAT(Box<pwat::PWAT>),
    QUST(Box<qust::QUST>),
    RACE(Box<race::RACE>),
    RADS(Box<rads::RADS>),
    RCCT(Box<rcct::RCCT>),
    RCPE(Box<rcpe::RCPE>),
    REFR(Box<refr::REFR>),
    REGN(Box<regn::REGN>),
    REPU(Box<repu::REPU>),
    RGDL(Box<rgdl::RGDL>),
    SCOL(Box<scol::SCOL>),
    SCPT(Box<scpt::SCPT>),
    SLPD(Box<slpd::SLPD>),
    SOUN(Box<soun::SOUN>),
    SPEL(Box<spel::SPEL>),
    STAT(Box<stat::STAT>),
    TACT(Box<tact::TACT>),
    TERM(Box<term::TERM>),
    TES4(Box<tes4::TES4>),
    TXST(Box<txst::TXST>),
    VTYP(Box<vtyp::VTYP>),
    WATR(Box<watr::WATR>),
    WEAP(Box<weap::WEAP>),
    WRLD(Box<wrld::WRLD>),
    WTHR(Box<wthr::WTHR>),
}

impl ParsedRecord {
    pub fn parse<'a, 'b>(record: &'a RawRecord<'b>) -> Result<Self, RecordParseError<'b>> {
        use crate::esp::record::sub::*;
        let parser = &mut RecordParser::new(record)?;

        Ok(match record.ty {
            ACHR => Self::ACHR(Box::new(Record::parse(parser)?)),
            ACRE => Self::ACRE(Box::new(Record::parse(parser)?)),
            ADDN => Self::ADDN(Box::new(Record::parse(parser)?)),
            ALCH => Self::ALCH(Box::new(Record::parse(parser)?)),
            ALOC => Self::ALOC(Box::new(Record::parse(parser)?)),
            AMEF => Self::AMEF(Box::new(Record::parse(parser)?)),
            AMMO => Self::AMMO(Box::new(Record::parse(parser)?)),
            ANIO => Self::ANIO(Box::new(Record::parse(parser)?)),
            ARMO => Self::ARMO(Box::new(Record::parse(parser)?)),
            ARMA => Self::ARMA(Box::new(Record::parse(parser)?)),
            ASPC => Self::ASPC(Box::new(Record::parse(parser)?)),
            AVIF => Self::AVIF(Box::new(Record::parse(parser)?)),
            BOOK => Self::BOOK(Box::new(Record::parse(parser)?)),
            BPTD => Self::BPTD(Box::new(Record::parse(parser)?)),
            CAMS => Self::CAMS(Box::new(Record::parse(parser)?)),
            CCRD => Self::CCRD(Box::new(Record::parse(parser)?)),
            CDCK => Self::CDCK(Box::new(Record::parse(parser)?)),
            CELL => Self::CELL(Box::new(Record::parse(parser)?)),
            CHAL => Self::CHAL(Box::new(Record::parse(parser)?)),
            CHIP => Self::CHIP(Box::new(Record::parse(parser)?)),
            CLAS => Self::CLAS(Box::new(Record::parse(parser)?)),
            CLMT => Self::CLMT(Box::new(Record::parse(parser)?)),
            CMNY => Self::CMNY(Box::new(Record::parse(parser)?)),
            COBJ => Self::COBJ(Box::new(Record::parse(parser)?)),
            CONT => Self::CONT(Box::new(Record::parse(parser)?)),
            CPTH => Self::CPTH(Box::new(Record::parse(parser)?)),
            CREA => Self::CREA(Box::new(Record::parse(parser)?)),
            CSNO => Self::CSNO(Box::new(Record::parse(parser)?)),
            CSTY => Self::CSTY(Box::new(Record::parse(parser)?)),
            DEBR => Self::DEBR(Box::new(Record::parse(parser)?)),
            DEHY => Self::DEHY(Box::new(Record::parse(parser)?)),
            DIAL => Self::DIAL(Box::new(Record::parse(parser)?)),
            DOBJ => Self::DOBJ(Box::new(Record::parse(parser)?)),
            DOOR => Self::DOOR(Box::new(Record::parse(parser)?)),
            ECZN => Self::ECZN(Box::new(Record::parse(parser)?)),
            EFSH => Self::EFSH(Box::new(Record::parse(parser)?)),
            ENCH => Self::ENCH(Box::new(Record::parse(parser)?)),
            EXPL => Self::EXPL(Box::new(Record::parse(parser)?)),
            EYES => Self::EYES(Box::new(Record::parse(parser)?)),
            FACT => Self::FACT(Box::new(Record::parse(parser)?)),
            FLST => Self::FLST(Box::new(Record::parse(parser)?)),
            FURN => Self::FURN(Box::new(Record::parse(parser)?)),
            GLOB => Self::GLOB(Box::new(Record::parse(parser)?)),
            GMST => Self::GMST(Box::new(Record::parse(parser)?)),
            GRAS => Self::GRAS(Box::new(Record::parse(parser)?)),
            HAIR => Self::HAIR(Box::new(Record::parse(parser)?)),
            HDPT => Self::HDPT(Box::new(Record::parse(parser)?)),
            HUNG => Self::HUNG(Box::new(Record::parse(parser)?)),
            IDLE => Self::IDLE(Box::new(Record::parse(parser)?)),
            IDLM => Self::IDLM(Box::new(Record::parse(parser)?)),
            IMGS => Self::IMGS(Box::new(Record::parse(parser)?)),
            IMAD => Self::IMAD(Box::new(Record::parse(parser)?)),
            IMOD => Self::IMOD(Box::new(Record::parse(parser)?)),
            INFO => Self::INFO(Box::new(Record::parse(parser)?)),
            INGR => Self::INGR(Box::new(Record::parse(parser)?)),
            IPCT => Self::IPCT(Box::new(Record::parse(parser)?)),
            IPDS => Self::IPDS(Box::new(Record::parse(parser)?)),
            KEYM => Self::KEYM(Box::new(Record::parse(parser)?)),
            LAND => Self::LAND(Box::new(Record::parse(parser)?)),
            LGTM => Self::LGTM(Box::new(Record::parse(parser)?)),
            LIGH => Self::LIGH(Box::new(Record::parse(parser)?)),
            LSCR => Self::LSCR(Box::new(Record::parse(parser)?)),
            LSCT => Self::LSCT(Box::new(Record::parse(parser)?)),
            LTEX => Self::LTEX(Box::new(Record::parse(parser)?)),
            LVLC => Self::LVLC(Box::new(Record::parse(parser)?)),
            LVLI => Self::LVLI(Box::new(Record::parse(parser)?)),
            LVLN => Self::LVLN(Box::new(Record::parse(parser)?)),
            MESG => Self::MESG(Box::new(Record::parse(parser)?)),
            MGEF => Self::MGEF(Box::new(Record::parse(parser)?)),
            MICN => Self::MICN(Box::new(Record::parse(parser)?)),
            MISC => Self::MISC(Box::new(Record::parse(parser)?)),
            NAVI => Self::NAVI(Box::new(Record::parse(parser)?)),
            NAVM => Self::NAVM(Box::new(Record::parse(parser)?)),
            NOTE => Self::NOTE(Box::new(Record::parse(parser)?)),
            NPC_ => Self::NPC_(Box::new(Record::parse(parser)?)),
            PACK => Self::PACK(Box::new(Record::parse(parser)?)),
            PERK => Self::PERK(Box::new(Record::parse(parser)?)),
            PGRE => Self::PGRE(Box::new(Record::parse(parser)?)),
            PMIS => Self::PMIS(Box::new(Record::parse(parser)?)),
            PROJ => Self::PROJ(Box::new(Record::parse(parser)?)),
            PWAT => Self::PWAT(Box::new(Record::parse(parser)?)),
            QUST => Self::QUST(Box::new(Record::parse(parser)?)),
            RACE => Self::RACE(Box::new(Record::parse(parser)?)),
            RADS => Self::RADS(Box::new(Record::parse(parser)?)),
            RCCT => Self::RCCT(Box::new(Record::parse(parser)?)),
            RCPE => Self::RCPE(Box::new(Record::parse(parser)?)),
            REFR => Self::REFR(Box::new(Record::parse(parser)?)),
            REGN => Self::REGN(Box::new(Record::parse(parser)?)),
            REPU => Self::REPU(Box::new(Record::parse(parser)?)),
            RGDL => Self::RGDL(Box::new(Record::parse(parser)?)),
            SCOL => Self::SCOL(Box::new(Record::parse(parser)?)),
            SCPT => Self::SCPT(Box::new(Record::parse(parser)?)),
            SLPD => Self::SLPD(Box::new(Record::parse(parser)?)),
            SOUN => Self::SOUN(Box::new(Record::parse(parser)?)),
            SPEL => Self::SPEL(Box::new(Record::parse(parser)?)),
            STAT => Self::STAT(Box::new(Record::parse(parser)?)),
            TACT => Self::TACT(Box::new(Record::parse(parser)?)),
            TERM => Self::TERM(Box::new(Record::parse(parser)?)),
            TES4 => Self::TES4(Box::new(Record::parse(parser)?)),
            TXST => Self::TXST(Box::new(Record::parse(parser)?)),
            VTYP => Self::VTYP(Box::new(Record::parse(parser)?)),
            WATR => Self::WATR(Box::new(Record::parse(parser)?)),
            WEAP => Self::WEAP(Box::new(Record::parse(parser)?)),
            WRLD => Self::WRLD(Box::new(Record::parse(parser)?)),
            WTHR => Self::WTHR(Box::new(Record::parse(parser)?)),
            ty => {
                return Err(RecordParseError::Custom(format!(
                    "Don't know how to parse \"{}\"",
                    ty
                )))
            }
        })
    }
}
