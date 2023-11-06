use std::any::Any;

use super::Record;

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
