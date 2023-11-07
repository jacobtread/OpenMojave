use super::RecordType;

pub mod actor_values;
pub mod condition;
pub mod destruction;
pub mod effect;
pub mod equipment_type;
pub mod item;
pub mod model;
pub mod object_bounds;
pub mod script;
pub mod skill;
pub mod sound_level;
pub mod xnam;

// Record type
pub const GRUP: RecordType = RecordType::new(b"GRUP");
pub const ACHR: RecordType = RecordType::new(b"ACHR");
pub const ACRE: RecordType = RecordType::new(b"ACRE");
pub const ACTI: RecordType = RecordType::new(b"ACTI");
pub const ADDN: RecordType = RecordType::new(b"ADDN");
pub const ALCH: RecordType = RecordType::new(b"ALCH");
pub const ALOC: RecordType = RecordType::new(b"ALOC");
pub const AMEF: RecordType = RecordType::new(b"AMEF");
pub const AMMO: RecordType = RecordType::new(b"AMMO");
pub const ANIO: RecordType = RecordType::new(b"ANIO");
pub const ARMO: RecordType = RecordType::new(b"ARMO");
pub const ARMA: RecordType = RecordType::new(b"ARMA");
pub const ASPC: RecordType = RecordType::new(b"ASPC");
pub const AVIF: RecordType = RecordType::new(b"AVIF");
pub const BOOK: RecordType = RecordType::new(b"BOOK");
pub const BPTD: RecordType = RecordType::new(b"BPTD");
pub const CAMS: RecordType = RecordType::new(b"CAMS");
pub const CCRD: RecordType = RecordType::new(b"CCRD");
pub const CDCK: RecordType = RecordType::new(b"CDCK");
pub const CELL: RecordType = RecordType::new(b"CELL");
pub const CHAL: RecordType = RecordType::new(b"CHAL");
pub const CHIP: RecordType = RecordType::new(b"CHIP");
pub const CLAS: RecordType = RecordType::new(b"CLAS");
pub const CLMT: RecordType = RecordType::new(b"CLMT");
pub const CMNY: RecordType = RecordType::new(b"CMNY");
pub const COBJ: RecordType = RecordType::new(b"COBJ");
pub const CONT: RecordType = RecordType::new(b"CONT");
pub const CPTH: RecordType = RecordType::new(b"CPTH");
pub const CREA: RecordType = RecordType::new(b"CREA");
pub const CSNO: RecordType = RecordType::new(b"CSNO");
pub const CSTY: RecordType = RecordType::new(b"CSTY");
pub const DEBR: RecordType = RecordType::new(b"DEBR");
pub const DEHY: RecordType = RecordType::new(b"DEHY");
pub const DIAL: RecordType = RecordType::new(b"DIAL");
pub const DOBJ: RecordType = RecordType::new(b"DOBJ");
pub const DOOR: RecordType = RecordType::new(b"DOOR");
pub const ECZN: RecordType = RecordType::new(b"ECZN");
pub const EFSH: RecordType = RecordType::new(b"EFSH");
pub const ENCH: RecordType = RecordType::new(b"ENCH");
pub const EXPL: RecordType = RecordType::new(b"EXPL");
pub const EYES: RecordType = RecordType::new(b"EYES");
pub const FACT: RecordType = RecordType::new(b"FACT");
pub const FLST: RecordType = RecordType::new(b"FLST");
pub const FURN: RecordType = RecordType::new(b"FURN");
pub const GLOB: RecordType = RecordType::new(b"GLOB");
pub const GMST: RecordType = RecordType::new(b"GMST");
pub const GRAS: RecordType = RecordType::new(b"GRAS");
pub const HAIR: RecordType = RecordType::new(b"HAIR");
pub const HDPT: RecordType = RecordType::new(b"HDPT");
pub const HUNG: RecordType = RecordType::new(b"HUNG");
pub const IDLE: RecordType = RecordType::new(b"IDLE");
pub const IDLM: RecordType = RecordType::new(b"IDLM");
pub const IMGS: RecordType = RecordType::new(b"IMGS");
pub const IMAD: RecordType = RecordType::new(b"IMAD");
pub const IMOD: RecordType = RecordType::new(b"IMOD");
pub const INFO: RecordType = RecordType::new(b"INFO");
pub const INGR: RecordType = RecordType::new(b"INGR");
pub const IPCT: RecordType = RecordType::new(b"IPCT");
pub const IPDS: RecordType = RecordType::new(b"IPDS");
pub const KEYM: RecordType = RecordType::new(b"KEYM");
pub const LAND: RecordType = RecordType::new(b"LAND");
pub const LGTM: RecordType = RecordType::new(b"LGTM");
pub const LIGH: RecordType = RecordType::new(b"LIGH");
pub const LSCR: RecordType = RecordType::new(b"LSCR");
pub const LSCT: RecordType = RecordType::new(b"LSCT");
pub const LTEX: RecordType = RecordType::new(b"LTEX");
pub const LVLC: RecordType = RecordType::new(b"LVLC");
pub const LVLI: RecordType = RecordType::new(b"LVLI");
pub const LVLN: RecordType = RecordType::new(b"LVLN");
pub const MESG: RecordType = RecordType::new(b"MESG");
pub const MGEF: RecordType = RecordType::new(b"MGEF");
pub const MICN: RecordType = RecordType::new(b"MICN");
pub const MISC: RecordType = RecordType::new(b"MISC");
pub const NAVI: RecordType = RecordType::new(b"NAVI");
pub const NAVM: RecordType = RecordType::new(b"NAVM");
pub const NOTE: RecordType = RecordType::new(b"NOTE");
pub const NPC_: RecordType = RecordType::new(b"NPC_");
pub const PACK: RecordType = RecordType::new(b"PACK");
pub const PERK: RecordType = RecordType::new(b"PERK");
pub const PGRE: RecordType = RecordType::new(b"PGRE");
pub const PMIS: RecordType = RecordType::new(b"PMIS");
pub const PROJ: RecordType = RecordType::new(b"PROJ");
pub const PWAT: RecordType = RecordType::new(b"PWAT");
pub const QUST: RecordType = RecordType::new(b"QUST");
pub const RACE: RecordType = RecordType::new(b"RACE");
pub const RADS: RecordType = RecordType::new(b"RADS");
pub const RCCT: RecordType = RecordType::new(b"RCCT");
pub const RCPE: RecordType = RecordType::new(b"RCPE");
pub const REFR: RecordType = RecordType::new(b"REFR");
pub const REGN: RecordType = RecordType::new(b"REGN");
pub const REPU: RecordType = RecordType::new(b"REPU");
pub const RGDL: RecordType = RecordType::new(b"RGDL");
pub const SCOL: RecordType = RecordType::new(b"SCOL");
pub const SCPT: RecordType = RecordType::new(b"SCPT");
pub const SLPD: RecordType = RecordType::new(b"SLPD");
pub const SOUN: RecordType = RecordType::new(b"SOUN");
pub const SPEL: RecordType = RecordType::new(b"SPEL");
pub const STAT: RecordType = RecordType::new(b"STAT");
pub const TACT: RecordType = RecordType::new(b"TACT");
pub const TERM: RecordType = RecordType::new(b"TERM");
pub const TES4: RecordType = RecordType::new(b"TES4");
pub const TXST: RecordType = RecordType::new(b"TXST");
pub const VTYP: RecordType = RecordType::new(b"VTYP");
pub const WATR: RecordType = RecordType::new(b"WATR");
pub const WEAP: RecordType = RecordType::new(b"WEAP");
pub const WRLD: RecordType = RecordType::new(b"WRLD");
pub const WTHR: RecordType = RecordType::new(b"WTHR");

// Sub record type

pub const HEDR: RecordType = RecordType::new(b"HEDR");
pub const MAST: RecordType = RecordType::new(b"MAST");
pub const OFST: RecordType = RecordType::new(b"OFST");
pub const DELE: RecordType = RecordType::new(b"DELE");
pub const NAME: RecordType = RecordType::new(b"NAME");
pub const CNAM: RecordType = RecordType::new(b"CNAM");
pub const SNAM: RecordType = RecordType::new(b"SNAM");
pub const ONAM: RecordType = RecordType::new(b"ONAM");
pub const EDID: RecordType = RecordType::new(b"EDID");
pub const DATA: RecordType = RecordType::new(b"DATA");
pub const XEZN: RecordType = RecordType::new(b"XEZN");
pub const OBND: RecordType = RecordType::new(b"OBND");
pub const TX00: RecordType = RecordType::new(b"TX00");
pub const TX01: RecordType = RecordType::new(b"TX01");
pub const TX02: RecordType = RecordType::new(b"TX02");
pub const TX03: RecordType = RecordType::new(b"TX03");
pub const TX04: RecordType = RecordType::new(b"TX04");
pub const TX05: RecordType = RecordType::new(b"TX05");
pub const DODT: RecordType = RecordType::new(b"DODT");
pub const ICON: RecordType = RecordType::new(b"ICON");
pub const MICO: RecordType = RecordType::new(b"MICO");
pub const FULL: RecordType = RecordType::new(b"FULL");
pub const DESC: RecordType = RecordType::new(b"DESC");
pub const ATTR: RecordType = RecordType::new(b"ATTR");
pub const XNAM: RecordType = RecordType::new(b"XNAM");
pub const RNAM: RecordType = RecordType::new(b"RNAM");
pub const MNAM: RecordType = RecordType::new(b"MNAM");
pub const FNAM: RecordType = RecordType::new(b"FNAM");
pub const INAM: RecordType = RecordType::new(b"INAM");
pub const HNAM: RecordType = RecordType::new(b"HNAM");
pub const WMI1: RecordType = RecordType::new(b"WMI1");
pub const MODL: RecordType = RecordType::new(b"MODL");
pub const MODB: RecordType = RecordType::new(b"MODB");
pub const MODT: RecordType = RecordType::new(b"MODT");
pub const MODS: RecordType = RecordType::new(b"MODS");
pub const MODD: RecordType = RecordType::new(b"MODD");
pub const MOD2: RecordType = RecordType::new(b"MOD2");
pub const MO2T: RecordType = RecordType::new(b"MO2T");
pub const MO2S: RecordType = RecordType::new(b"MO2S");
pub const MOD3: RecordType = RecordType::new(b"MOD3");
pub const MO3T: RecordType = RecordType::new(b"MO3T");
pub const MO3S: RecordType = RecordType::new(b"MO3S");
pub const MOSD: RecordType = RecordType::new(b"MOSD");
pub const MOD4: RecordType = RecordType::new(b"MOD4");
pub const MO4T: RecordType = RecordType::new(b"MO4T");
pub const MO4S: RecordType = RecordType::new(b"MO4S");
pub const INDX: RecordType = RecordType::new(b"INDX");
pub const YNAM: RecordType = RecordType::new(b"YNAM");
pub const NAM2: RecordType = RecordType::new(b"NAM2");
pub const VTCK: RecordType = RecordType::new(b"VTCK");
pub const PNAM: RecordType = RecordType::new(b"PNAM");
pub const UNAM: RecordType = RecordType::new(b"UNAM");
pub const NAM0: RecordType = RecordType::new(b"NAM0");
pub const NAM1: RecordType = RecordType::new(b"NAM1");
pub const ENAM: RecordType = RecordType::new(b"ENAM");
pub const FGGS: RecordType = RecordType::new(b"FGGS");
pub const FGGA: RecordType = RecordType::new(b"FGGA");
pub const FGTS: RecordType = RecordType::new(b"FGTS");
pub const SNDD: RecordType = RecordType::new(b"SNDD");
pub const SNDX: RecordType = RecordType::new(b"SNDX");
pub const ANAM: RecordType = RecordType::new(b"ANAM");
pub const GNAM: RecordType = RecordType::new(b"GNAM");
pub const WNAM: RecordType = RecordType::new(b"WNAM");
pub const RDAT: RecordType = RecordType::new(b"RDAT");
pub const SLSD: RecordType = RecordType::new(b"SLSD");
pub const SCVR: RecordType = RecordType::new(b"SCVR");
pub const SCHR: RecordType = RecordType::new(b"SCHR");
pub const SCDA: RecordType = RecordType::new(b"SCDA");
pub const SCTX: RecordType = RecordType::new(b"SCTX");
pub const SCRO: RecordType = RecordType::new(b"SCRO");
pub const TNAM: RecordType = RecordType::new(b"TNAM");
pub const EFIT: RecordType = RecordType::new(b"EFIT");
pub const CTDA: RecordType = RecordType::new(b"CTDA");
pub const ENIT: RecordType = RecordType::new(b"ENIT");
pub const SPIT: RecordType = RecordType::new(b"SPIT");
pub const DSTD: RecordType = RecordType::new(b"DSTD");
pub const DMDL: RecordType = RecordType::new(b"DMDL");
pub const DMDT: RecordType = RecordType::new(b"DMDT");
pub const DSTF: RecordType = RecordType::new(b"DSTF");
pub const DEST: RecordType = RecordType::new(b"DEST");
pub const SCRI: RecordType = RecordType::new(b"SCRI");
pub const VNAM: RecordType = RecordType::new(b"VNAM");
pub const XATO: RecordType = RecordType::new(b"XATO");
pub const ITXT: RecordType = RecordType::new(b"ITXT");
pub const EITM: RecordType = RecordType::new(b"EITM");
pub const BMDT: RecordType = RecordType::new(b"BMDT");
pub const ICO2: RecordType = RecordType::new(b"ICO2");
pub const MIC2: RecordType = RecordType::new(b"MIC2");
pub const BMCT: RecordType = RecordType::new(b"BMCT");
pub const REPL: RecordType = RecordType::new(b"REPL");
pub const BIPL: RecordType = RecordType::new(b"BIPL");
pub const ETYP: RecordType = RecordType::new(b"ETYP");
pub const ZNAM: RecordType = RecordType::new(b"ZNAM");
pub const BNAM: RecordType = RecordType::new(b"BNAM");
pub const CNTO: RecordType = RecordType::new(b"CNTO");
pub const COED: RecordType = RecordType::new(b"COED");
pub const QNAM: RecordType = RecordType::new(b"QNAM");
pub const BRUS: RecordType = RecordType::new(b"BRUS");
pub const XRGD: RecordType = RecordType::new(b"XRGD");
pub const XRGB: RecordType = RecordType::new(b"XRGB");
pub const XPRD: RecordType = RecordType::new(b"XPRD");
pub const XPPA: RecordType = RecordType::new(b"XPPA");
pub const XLCM: RecordType = RecordType::new(b"XLCM");
pub const XMRC: RecordType = RecordType::new(b"XMRC");
pub const XCNT: RecordType = RecordType::new(b"XCNT");
pub const XRDS: RecordType = RecordType::new(b"XRDS");
pub const XHLP: RecordType = RecordType::new(b"XHLP");
pub const XDCR: RecordType = RecordType::new(b"XDCR");
pub const XLKR: RecordType = RecordType::new(b"XLKR");
pub const XCLP: RecordType = RecordType::new(b"XCLP");
pub const XADP: RecordType = RecordType::new(b"XADP");
pub const XAPR: RecordType = RecordType::new(b"XAPR");
pub const XESP: RecordType = RecordType::new(b"XESP");
pub const XEMI: RecordType = RecordType::new(b"XEMI");
pub const XMBR: RecordType = RecordType::new(b"XMBR");
pub const XIBS: RecordType = RecordType::new(b"XIBS");
pub const XSCL: RecordType = RecordType::new(b"XSCL");
pub const XOWN: RecordType = RecordType::new(b"XOWN");
pub const XRNK: RecordType = RecordType::new(b"XRNK");
pub const CARD: RecordType = RecordType::new(b"CARD");
pub const LNAM: RecordType = RecordType::new(b"LNAM");
pub const FLTV: RecordType = RecordType::new(b"FLTV");
pub const NAM3: RecordType = RecordType::new(b"NAM3");
pub const NAM4: RecordType = RecordType::new(b"NAM4");
pub const NAM5: RecordType = RecordType::new(b"NAM5");
pub const NAM6: RecordType = RecordType::new(b"NAM6");
pub const NAM7: RecordType = RecordType::new(b"NAM7");
pub const NAM8: RecordType = RecordType::new(b"NAM8");
pub const NAM9: RecordType = RecordType::new(b"NAM9");
pub const RCIL: RecordType = RecordType::new(b"RCIL");
pub const RCQY: RecordType = RecordType::new(b"RCQY");
pub const RCOD: RecordType = RecordType::new(b"RCOD");
pub const DNAM: RecordType = RecordType::new(b"DNAM");
pub const DAT2: RecordType = RecordType::new(b"DAT2");
pub const INTV: RecordType = RecordType::new(b"INTV");
