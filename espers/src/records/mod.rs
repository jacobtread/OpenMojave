pub mod aact;
pub mod achr;
pub mod acti;
pub mod addn;
pub mod alch;
pub mod ammo;
pub mod anio;
pub mod appa;
pub mod arma;
pub mod armo;
pub mod arto;
pub mod aspc;
pub mod astp;
pub mod avif;
pub mod book;
pub mod bptd;
pub mod cams;
pub mod cell;
pub mod clas;
pub mod clfm;
pub mod clmt;
pub mod cobj;
pub mod coll;
pub mod cont;
pub mod cpth;
pub mod csty;
pub mod debr;
pub mod dial;
pub mod dlbr;
pub mod dlvw;
pub mod dobj;
pub mod door;
pub mod dual;
pub mod eczn;
pub mod efsh;
pub mod ench;
pub mod equp;
pub mod expl;
pub mod eyes;
pub mod fact;
pub mod flor;
pub mod flst;
pub mod fstp;
pub mod fsts;
pub mod furn;
pub mod glob;
pub mod gmst;
pub mod gras;
pub mod grup;
pub mod hazd;
pub mod hdpt;
pub mod idle;
pub mod idlm;
pub mod imad;
pub mod imgs;
pub mod info;
pub mod ingr;
pub mod ipct;
pub mod ipds;
pub mod keym;
pub mod kywd;
pub mod land;
pub mod lcrt;
pub mod lctn;
pub mod lgtm;
pub mod ligh;
pub mod lscr;
pub mod ltex;
pub mod lvli;
pub mod lvln;
pub mod lvsp;
pub mod mato;
pub mod matt;
pub mod mesg;
pub mod mgef;
pub mod misc;
pub mod movt;
pub mod mstt;
pub mod musc;
pub mod must;
pub mod navi;
pub mod navm;
pub mod note;
pub mod npc_;
pub mod otft;
pub mod pack;
pub mod perk;
pub mod pgre;
pub mod phzd;
pub mod proj;
pub mod qust;
pub mod race;
pub mod refr;
pub mod regn;
pub mod rela;
pub mod revb;
pub mod rfct;
pub mod scen;
pub mod scrl;
pub mod shou;
pub mod slgm;
pub mod smbn;
pub mod smen;
pub mod smqn;
pub mod snct;
pub mod sndr;
pub mod sopm;
pub mod soun;
pub mod spel;
pub mod spgd;
pub mod stat;
pub mod tact;
pub mod tes4;
pub mod tree;
pub mod txst;
pub mod unknown;
pub mod vtyp;
pub mod watr;
pub mod weap;
pub mod woop;
pub mod wrld;
pub mod wthr;

pub use self::glob::{GlobalVariable, GLOB};
use self::unknown::{Unknown, UNKNOWN};
pub use aact::{Action, AACT};
pub use achr::{ActorRef, ACHR};
pub use acti::{Activator, ACTI};
pub use addn::{AddonNode, AddonNodeFlags, ADDN};
pub use alch::{Alchemy, ALCH};
pub use ammo::{Ammo, AMMO};
pub use anio::{AnimatedObjectInfo, ANIO};
pub use appa::{Apparatus, APPA};
pub use arma::{ArmorAddon, ARMA};
pub use armo::{Armor, ARMO};
pub use arto::{ArtObject, ARTO};
pub use aspc::{AcousticSpace, ASPC};
pub use astp::{AssociationType, ASTP};
pub use avif::{ActorValue, AVIF};
pub use book::{Book, BookData, BookFlags, BOOK};
pub use bptd::{BodyPartData, BPTD};
pub use cams::{CameraShot, CAMS};
pub use cell::{Cell, CELL};
pub use clas::{Class, CLAS};
pub use clfm::{Color, CLFM};
pub use clmt::{Climate, CLMT};
pub use cobj::{ConstructibleObj, COBJ};
pub use coll::{CollisionLayer, COLL};
pub use cont::{Container, CONT};
pub use cpth::{CameraPath, CPTH};
pub use csty::{CombatStyle, CSTY};
pub use debr::{Debris, DEBR};
pub use dial::{DialogueTopic, DIAL};
pub use dlbr::{DialogueBranch, DLBR};
pub use dlvw::{DialogueView, DLVW};
pub use dobj::{DefaultObjectManager, DOBJ};
pub use door::{Door, DOOR};
pub use dual::{DualCastArt, DUAL};
pub use eczn::{EncounterZone, ECZN};
pub use efsh::{EffectShader, EFSH};
pub use ench::{Enchantment, ENCH};
pub use equp::{EquipSlot, EQUP};
pub use expl::{Explosion, EXPL};
pub use eyes::{Eyes, EYES};
pub use fact::{Faction, FACT};
pub use flor::{Flora, FLOR};
pub use flst::{FormList, FLST};
pub use fstp::{Footstep, FSTP};
pub use fsts::{FootstepSet, FSTS};
pub use furn::{Furniture, FURN};
pub use gmst::{GameSetting, GMST};
pub use gras::{Grass, GRAS};
pub use grup::{Group, GRUP};
pub use hazd::{Hazard, HAZD};
pub use hdpt::{HeadPart, HDPT};
pub use idle::{IdleAnimation, IDLE};
pub use idlm::{IdleMarker, IDLM};
pub use imad::{ImageSpaceModifier, IMAD};
pub use imgs::{ImageSpace, IMGS};
pub use info::{DialogueTopicInfo, INFO};
pub use ingr::{Ingredient, INGR};
pub use ipct::{ImpactData, IPCT};
pub use ipds::{ImpactDataSet, IPDS};
pub use keym::{Key, KEYM};
pub use kywd::{Keyword, KYWD};
pub use land::{Landscape, LAND};
pub use lcrt::{LocationRef, LCRT};
pub use lctn::{Location, LCTN};
pub use lgtm::{LightingTemplate, LGTM};
pub use ligh::{Light, LIGH};
pub use lscr::{LoadScreen, LSCR};
pub use ltex::{LandTexture, LTEX};
pub use lvli::{LeveledItem, LVLI};
pub use lvln::{LeveledActor, LVLN};
pub use lvsp::{LeveledSpell, LVSP};
pub use mato::{MaterialObject, MATO};
pub use matt::{MaterialType, MATT};
pub use mesg::{Message, MESG};
pub use mgef::{MagicEffect, MGEF};
pub use misc::{MiscItem, MISC};
pub use movt::{MovementType, MOVT};
pub use mstt::{MovableStatic, MSTT};
pub use musc::{MusicType, MUSC};
pub use must::{MusicTrack, MUST};
pub use navi::{Navigation, NAVI};
pub use navm::{NavMesh, NAVM};
pub use note::{Note, NOTE};
pub use npc_::{NPC, NPC_};
pub use otft::{Outfit, OTFT};
pub use pack::{AIPackage, PACK};
pub use perk::{Perk, PERK};
pub use pgre::{Placedgrenade, PGRE};
pub use phzd::{Placedhazard, PHZD};
pub use proj::{Projectile, PROJ};
pub use qust::{Quest, QUST};
pub use race::{Race, RACE};
pub use refr::{ObjectRef, REFR};
pub use regn::{Region, REGN};
pub use rela::{Relationship, RELA};
pub use revb::{ReverbParameters, REVB};
pub use rfct::{VisualEffect, RFCT};
pub use scen::{Scene, SCEN};
pub use scrl::{Scroll, SCRL};
pub use shou::{Shout, SHOU};
pub use slgm::{SoulGem, SLGM};
pub use smbn::{StoryManagerBranchNode, SMBN};
pub use smen::{StoryManagerEventNode, SMEN};
pub use smqn::{StoryManagerQuestNode, SMQN};
pub use snct::{SoundCategory, SNCT};
pub use sndr::{SoundReference, SNDR};
pub use sopm::{SoundOutputModel, SOPM};
pub use soun::{Sound, SOUN};
pub use spel::{Spell, SPEL};
pub use spgd::{ShaderParticleGeometry, SPGD};
pub use stat::{Static, STAT};
pub use tact::{TalkingActivator, TACT};
pub use tes4::{Header, TES4};
pub use tree::{Tree, TREE};
pub use txst::{TextureSet, TXST};
pub use vtyp::{VoiceType, VTYP};
pub use watr::{WaterType, WATR};
pub use weap::{Weapon, WEAP};
pub use woop::{WordOfPower, WOOP};
pub use wrld::{World, WRLD};
pub use wthr::{Weather, WTHR};

use crate::error::Error;
use crate::string_table::StringTables;

use binrw::binrw;
use bitflags::bitflags;
use flate2::read::ZlibDecoder;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::{Cursor, Read};

bitflags! {
    #[binrw]
    #[derive(Serialize, Deserialize)]
    pub struct Flags: u32 {
        const MASTER = 0x00000001;
        const DELETED_GROUP = 0x00000010;
        const DELETED_RECORD = 0x00000010;
        const LOCALIZED = 0x00000080;
        const LIGHT_MASTER = 0x00000200;
        const COMPRESSED = 0x00040000;
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordHeader {
    pub size: u32,
    pub flags: Flags,
    pub form_id: u32,
    pub timestamp: u16,
    pub version_control: u16,
    pub internal_version: u16,
    pub unknown: u16,
}

#[binrw]
#[brw(little)]
#[br(import(localized: bool))]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RawRecord {
    AACT(AACT),
    ACHR(ACHR),
    ACTI(#[br(args(localized))] ACTI),
    ADDN(ADDN),
    ALCH(#[br(args(localized))] ALCH),
    AMMO(#[br(args(localized))] AMMO),
    ANIO(ANIO),
    APPA(#[br(args(localized))] APPA),
    ARMA(ARMA),
    ARMO(#[br(args(localized))] ARMO),
    ARTO(ARTO),
    ASPC(ASPC),
    ASTP(ASTP),
    AVIF(AVIF),
    BOOK(#[br(args(localized))] BOOK),
    BPTD(BPTD),
    CAMS(CAMS),
    CELL(CELL),
    CLAS(#[br(args(localized))] CLAS),
    CLFM(#[br(args(localized))] CLFM),
    CLMT(CLMT),
    COBJ(COBJ),
    COLL(#[br(args(localized))] COLL),
    CONT(CONT),
    CPTH(CPTH),
    CSTY(CSTY),
    DEBR(DEBR),
    DIAL(DIAL),
    DLBR(DLBR),
    DLVW(DLVW),
    DOBJ(DOBJ),
    DOOR(#[br(args(localized))] DOOR),
    DUAL(DUAL),
    ECZN(ECZN),
    EFSH(EFSH),
    ENCH(#[br(args(localized))] ENCH),
    EQUP(EQUP),
    EXPL(#[br(args(localized))] EXPL),
    EYES(#[br(args(localized))] EYES),
    FACT(FACT),
    FLOR(#[br(args(localized))] FLOR),
    FLST(FLST),
    FSTP(FSTP),
    FSTS(FSTS),
    FURN(FURN),
    GLOB(GLOB),
    GMST(#[br(args(localized))] GMST),
    GRAS(GRAS),
    GRUP(#[br(args(localized))] GRUP),
    HAZD(HAZD),
    HDPT(HDPT),
    IDLE(IDLE),
    IDLM(IDLM),
    IMAD(IMAD),
    IMGS(IMGS),
    INFO(INFO),
    INGR(INGR),
    IPCT(IPCT),
    IPDS(IPDS),
    KEYM(KEYM),
    KYWD(KYWD),
    LAND(LAND),
    LCRT(LCRT),
    LCTN(LCTN),
    LGTM(LGTM),
    LIGH(LIGH),
    LSCR(LSCR),
    LTEX(LTEX),
    LVLI(LVLI),
    LVLN(LVLN),
    LVSP(LVSP),
    MATO(MATO),
    MATT(MATT),
    MESG(MESG),
    MGEF(#[br(args(localized))] MGEF),
    MISC(MISC),
    MOVT(MOVT),
    MSTT(MSTT),
    MUSC(MUSC),
    MUST(MUST),
    NAVI(NAVI),
    NAVM(NAVM),
    NOTE(NOTE),
    NPC_(NPC_),
    OTFT(OTFT),
    PACK(PACK),
    PERK(PERK),
    PGRE(PGRE),
    PHZD(PHZD),
    PROJ(PROJ),
    QUST(QUST),
    RACE(RACE),
    REFR(REFR),
    REGN(REGN),
    RELA(RELA),
    REVB(REVB),
    RFCT(RFCT),
    SCEN(SCEN),
    SCRL(SCRL),
    SHOU(SHOU),
    SLGM(SLGM),
    SMBN(SMBN),
    SMEN(SMEN),
    SMQN(SMQN),
    SNCT(SNCT),
    SNDR(SNDR),
    SOPM(SOPM),
    SOUN(SOUN),
    SPEL(SPEL),
    SPGD(SPGD),
    STAT(STAT),
    TACT(TACT),
    TREE(TREE),
    TXST(TXST),
    VTYP(VTYP),
    WATR(WATR),
    WEAP(WEAP),
    WOOP(WOOP),
    WRLD(WRLD),
    WTHR(WTHR),
    UNKNOWN(UNKNOWN),
}

impl RawRecord {
    fn magic(&self) -> &str {
        match self {
            RawRecord::ACHR(x) => "ACHR",
            RawRecord::AACT(x) => "AACT",
            RawRecord::ACTI(x) => "ACTI",
            RawRecord::ADDN(x) => "ADDN",
            RawRecord::ALCH(x) => "ALCH",
            RawRecord::AMMO(x) => "AMMO",
            RawRecord::ANIO(x) => "ANIO",
            RawRecord::APPA(x) => "APPA",
            RawRecord::ARMA(x) => "ARMA",
            RawRecord::ARMO(x) => "ARMO",
            RawRecord::ARTO(x) => "ARTO",
            RawRecord::ASPC(x) => "ASPC",
            RawRecord::ASTP(x) => "ASTP",
            RawRecord::AVIF(x) => "AVIF",
            RawRecord::BOOK(x) => "BOOK",
            RawRecord::BPTD(x) => "BPTD",
            RawRecord::CAMS(x) => "CAMS",
            RawRecord::CELL(x) => "CELL",
            RawRecord::CLAS(x) => "CLAS",
            RawRecord::CLFM(x) => "CLFM",
            RawRecord::CLMT(x) => "CLMT",
            RawRecord::COBJ(x) => "COBJ",
            RawRecord::COLL(x) => "COLL",
            RawRecord::CONT(x) => "CONT",
            RawRecord::CPTH(x) => "CPTH",
            RawRecord::CSTY(x) => "CSTY",
            RawRecord::DEBR(x) => "DEBR",
            RawRecord::DIAL(x) => "DIAL",
            RawRecord::DLBR(x) => "DLBR",
            RawRecord::DLVW(x) => "DLVW",
            RawRecord::DOBJ(x) => "DOBJ",
            RawRecord::DOOR(x) => "DOOR",
            RawRecord::DUAL(x) => "DUAL",
            RawRecord::ECZN(x) => "ECZN",
            RawRecord::EFSH(x) => "EFSH",
            RawRecord::ENCH(x) => "ENCH",
            RawRecord::EQUP(x) => "EQUP",
            RawRecord::EXPL(x) => "EXPL",
            RawRecord::EYES(x) => "EYES",
            RawRecord::FACT(x) => "FACT",
            RawRecord::FLOR(x) => "FLOR",
            RawRecord::FLST(x) => "FLST",
            RawRecord::FSTP(x) => "FSTP",
            RawRecord::FSTS(x) => "FSTS",
            RawRecord::FURN(x) => "FURN",
            RawRecord::GLOB(x) => "GLOB",
            RawRecord::GMST(x) => "GMST",
            RawRecord::GRAS(x) => "GRAS",
            RawRecord::GRUP(x) => "GRUP",
            RawRecord::HAZD(x) => "HAZD",
            RawRecord::HDPT(x) => "HDPT",
            RawRecord::IDLE(x) => "IDLE",
            RawRecord::IDLM(x) => "IDLM",
            RawRecord::IMAD(x) => "IMAD",
            RawRecord::IMGS(x) => "IMGS",
            RawRecord::INFO(x) => "INFO",
            RawRecord::INGR(x) => "INGR",
            RawRecord::IPCT(x) => "IPCT",
            RawRecord::IPDS(x) => "IPDS",
            RawRecord::KEYM(x) => "KEYM",
            RawRecord::KYWD(x) => "KYWD",
            RawRecord::LAND(x) => "LAND",
            RawRecord::LCRT(x) => "LCRT",
            RawRecord::LCTN(x) => "LCTN",
            RawRecord::LGTM(x) => "LGTM",
            RawRecord::LIGH(x) => "LIGH",
            RawRecord::LSCR(x) => "LSCR",
            RawRecord::LTEX(x) => "LTEX",
            RawRecord::LVLI(x) => "LVLI",
            RawRecord::LVLN(x) => "LVLN",
            RawRecord::LVSP(x) => "LVSP",
            RawRecord::MATO(x) => "MATO",
            RawRecord::MATT(x) => "MATT",
            RawRecord::MESG(x) => "MESG",
            RawRecord::MGEF(x) => "MGEF",
            RawRecord::MISC(x) => "MISC",
            RawRecord::MOVT(x) => "MOVT",
            RawRecord::MSTT(x) => "MSTT",
            RawRecord::MUSC(x) => "MUSC",
            RawRecord::MUST(x) => "MUST",
            RawRecord::NAVI(x) => "NAVI",
            RawRecord::NAVM(x) => "NAVM",
            RawRecord::NOTE(x) => "NOTE",
            RawRecord::NPC_(x) => "NPC_",
            RawRecord::OTFT(x) => "OTFT",
            RawRecord::PACK(x) => "PACK",
            RawRecord::PERK(x) => "PERK",
            RawRecord::PGRE(x) => "PGRE",
            RawRecord::PHZD(x) => "PHZD",
            RawRecord::PROJ(x) => "PROJ",
            RawRecord::QUST(x) => "QUST",
            RawRecord::RACE(x) => "RACE",
            RawRecord::REFR(x) => "REFR",
            RawRecord::REGN(x) => "REGN",
            RawRecord::RELA(x) => "RELA",
            RawRecord::REVB(x) => "REVB",
            RawRecord::RFCT(x) => "RFCT",
            RawRecord::SCEN(x) => "SCEN",
            RawRecord::SCRL(x) => "SCRL",
            RawRecord::SHOU(x) => "SHOU",
            RawRecord::SLGM(x) => "SLGM",
            RawRecord::SMBN(x) => "SMBN",
            RawRecord::SMEN(x) => "SMEN",
            RawRecord::SMQN(x) => "SMQN",
            RawRecord::SNCT(x) => "SNCT",
            RawRecord::SNDR(x) => "SNDR",
            RawRecord::SOPM(x) => "SOPM",
            RawRecord::SOUN(x) => "SOUN",
            RawRecord::SPEL(x) => "SPEL",
            RawRecord::SPGD(x) => "SPGD",
            RawRecord::STAT(x) => "STAT",
            RawRecord::TACT(x) => "TACT",
            RawRecord::TREE(x) => "TREE",
            RawRecord::TXST(x) => "TXST",
            RawRecord::VTYP(x) => "VTYP",
            RawRecord::WATR(x) => "WATR",
            RawRecord::WEAP(x) => "WEAP",
            RawRecord::WOOP(x) => "WOOP",
            RawRecord::WRLD(x) => "WRLD",
            RawRecord::WTHR(x) => "WTHR",
            RawRecord::UNKNOWN(x) => "UNKN",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Record {
    AIPackage(AIPackage),
    AcousticSpace(AcousticSpace),
    Action(Action),
    Activator(Activator),
    ActorRef(ActorRef),
    ActorValue(ActorValue),
    AddonNode(AddonNode),
    Alchemy(Alchemy),
    Ammo(Ammo),
    AnimatedObjectInfo(AnimatedObjectInfo),
    Apparatus(Apparatus),
    Armor(Armor),
    ArmorAddon(ArmorAddon),
    ArtObject(ArtObject),
    AssociationType(AssociationType),
    BodyPartData(BodyPartData),
    Book(Book),
    CameraPath(CameraPath),
    CameraShot(CameraShot),
    Cell(Cell),
    Class(Class),
    Climate(Climate),
    CollisionLayer(CollisionLayer),
    Color(Color),
    CombatStyle(CombatStyle),
    ConstructibleObj(ConstructibleObj),
    Container(Container),
    Debris(Debris),
    DefaultObjectManager(DefaultObjectManager),
    DialogueBranch(DialogueBranch),
    DialogueTopic(DialogueTopic),
    DialogueTopicInfo(DialogueTopicInfo),
    DialogueView(DialogueView),
    Door(Door),
    DualCastArt(DualCastArt),
    EffectShader(EffectShader),
    Enchantment(Enchantment),
    EncounterZone(EncounterZone),
    EquipSlot(EquipSlot),
    Explosion(Explosion),
    Eyes(Eyes),
    Faction(Faction),
    Flora(Flora),
    Footstep(Footstep),
    FootstepSet(FootstepSet),
    FormList(FormList),
    Furniture(Furniture),
    GameSetting(GameSetting),
    GlobalVariable(GlobalVariable),
    Grass(Grass),
    Group(Group),
    Hazard(Hazard),
    HeadPart(HeadPart),
    IdleAnimation(IdleAnimation),
    IdleMarker(IdleMarker),
    ImageSpace(ImageSpace),
    ImageSpaceModifier(ImageSpaceModifier),
    ImpactData(ImpactData),
    ImpactDataSet(ImpactDataSet),
    Ingredient(Ingredient),
    Key(Key),
    Keyword(Keyword),
    LandTexture(LandTexture),
    Landscape(Landscape),
    LeveledActor(LeveledActor),
    LeveledItem(LeveledItem),
    LeveledSpell(LeveledSpell),
    Light(Light),
    LightingTemplate(LightingTemplate),
    LoadScreen(LoadScreen),
    Location(Location),
    LocationRef(LocationRef),
    MagicEffect(MagicEffect),
    MaterialObject(MaterialObject),
    MaterialType(MaterialType),
    Message(Message),
    MiscItem(MiscItem),
    MovableStatic(MovableStatic),
    MovementType(MovementType),
    MusicTrack(MusicTrack),
    MusicType(MusicType),
    NPC(NPC),
    NavMesh(NavMesh),
    Navigation(Navigation),
    Note(Note),
    ObjectRef(ObjectRef),
    Outfit(Outfit),
    Perk(Perk),
    Placedgrenade(Placedgrenade),
    Placedhazard(Placedhazard),
    Projectile(Projectile),
    Quest(Quest),
    Race(Race),
    Region(Region),
    Relationship(Relationship),
    ReverbParameters(ReverbParameters),
    Scene(Scene),
    Scroll(Scroll),
    ShaderParticleGeometry(ShaderParticleGeometry),
    Shout(Shout),
    SoulGem(SoulGem),
    Sound(Sound),
    SoundCategory(SoundCategory),
    SoundOutputModel(SoundOutputModel),
    SoundReference(SoundReference),
    Spell(Spell),
    Static(Static),
    StoryManagerBranchNode(StoryManagerBranchNode),
    StoryManagerEventNode(StoryManagerEventNode),
    StoryManagerQuestNode(StoryManagerQuestNode),
    TalkingActivator(TalkingActivator),
    TextureSet(TextureSet),
    Tree(Tree),
    VisualEffect(VisualEffect),
    VoiceType(VoiceType),
    WaterType(WaterType),
    Weapon(Weapon),
    Weather(Weather),
    WordOfPower(WordOfPower),
    World(World),
    Unknown(Unknown),
}

impl TryFrom<RawRecord> for Record {
    type Error = Error;

    fn try_from(r: RawRecord) -> Result<Self, Self::Error> {
        println!("{}", r.magic());
        match r {
            RawRecord::AACT(x) => Ok(Record::Action(Action::try_from(x)?)),
            RawRecord::ACHR(x) => Ok(Record::ActorRef(ActorRef::try_from(x)?)),
            RawRecord::ACTI(x) => Ok(Record::Activator(Activator::try_from(x)?)),
            RawRecord::ADDN(x) => Ok(Record::AddonNode(AddonNode::try_from(x)?)),
            RawRecord::ALCH(x) => Ok(Record::Alchemy(Alchemy::try_from(x)?)),
            RawRecord::AMMO(x) => Ok(Record::Ammo(Ammo::try_from(x)?)),
            RawRecord::ANIO(x) => Ok(Record::AnimatedObjectInfo(AnimatedObjectInfo::try_from(x)?)),
            RawRecord::APPA(x) => Ok(Record::Apparatus(Apparatus::try_from(x)?)),
            RawRecord::ARMA(x) => Ok(Record::ArmorAddon(ArmorAddon::try_from(x)?)),
            RawRecord::ARMO(x) => Ok(Record::Armor(Armor::try_from(x)?)),
            RawRecord::ARTO(x) => Ok(Record::ArtObject(ArtObject::try_from(x)?)),
            RawRecord::ASPC(x) => Ok(Record::AcousticSpace(AcousticSpace::try_from(x)?)),
            RawRecord::ASTP(x) => Ok(Record::AssociationType(AssociationType::try_from(x)?)),
            RawRecord::AVIF(x) => Ok(Record::ActorValue(ActorValue::try_from(x)?)),
            RawRecord::BOOK(x) => Ok(Record::Book(Book::try_from(x)?)),
            RawRecord::BPTD(x) => Ok(Record::BodyPartData(BodyPartData::try_from(x)?)),
            RawRecord::CAMS(x) => Ok(Record::CameraShot(CameraShot::try_from(x)?)),
            RawRecord::CELL(x) => Ok(Record::Cell(Cell::try_from(x)?)),
            RawRecord::CLAS(x) => Ok(Record::Class(Class::try_from(x)?)),
            RawRecord::CLFM(x) => Ok(Record::Color(Color::try_from(x)?)),
            RawRecord::CLMT(x) => Ok(Record::Climate(Climate::try_from(x)?)),
            RawRecord::COBJ(x) => Ok(Record::ConstructibleObj(ConstructibleObj::try_from(x)?)),
            RawRecord::COLL(x) => Ok(Record::CollisionLayer(CollisionLayer::try_from(x)?)),
            RawRecord::CONT(x) => Ok(Record::Container(Container::try_from(x)?)),
            RawRecord::CPTH(x) => Ok(Record::CameraPath(CameraPath::try_from(x)?)),
            RawRecord::CSTY(x) => Ok(Record::CombatStyle(CombatStyle::try_from(x)?)),
            RawRecord::DEBR(x) => Ok(Record::Debris(Debris::try_from(x)?)),
            RawRecord::DIAL(x) => Ok(Record::DialogueTopic(DialogueTopic::try_from(x)?)),
            RawRecord::DLBR(x) => Ok(Record::DialogueBranch(DialogueBranch::try_from(x)?)),
            RawRecord::DLVW(x) => Ok(Record::DialogueView(DialogueView::try_from(x)?)),
            RawRecord::DOBJ(x) => Ok(Record::DefaultObjectManager(
                DefaultObjectManager::try_from(x)?,
            )),
            RawRecord::DOOR(x) => Ok(Record::Door(Door::try_from(x)?)),
            RawRecord::DUAL(x) => Ok(Record::DualCastArt(DualCastArt::try_from(x)?)),
            RawRecord::ECZN(x) => Ok(Record::EncounterZone(EncounterZone::try_from(x)?)),
            RawRecord::EFSH(x) => Ok(Record::EffectShader(EffectShader::try_from(x)?)),
            RawRecord::ENCH(x) => Ok(Record::Enchantment(Enchantment::try_from(x)?)),
            RawRecord::EQUP(x) => Ok(Record::EquipSlot(EquipSlot::try_from(x)?)),
            RawRecord::EXPL(x) => Ok(Record::Explosion(Explosion::try_from(x)?)),
            RawRecord::EYES(x) => Ok(Record::Eyes(Eyes::try_from(x)?)),
            RawRecord::FACT(x) => Ok(Record::Faction(Faction::try_from(x)?)),
            RawRecord::FLOR(x) => Ok(Record::Flora(Flora::try_from(x)?)),
            RawRecord::FLST(x) => Ok(Record::FormList(FormList::try_from(x)?)),
            RawRecord::FSTP(x) => Ok(Record::Footstep(Footstep::try_from(x)?)),
            RawRecord::FSTS(x) => Ok(Record::FootstepSet(FootstepSet::try_from(x)?)),
            RawRecord::FURN(x) => Ok(Record::Furniture(Furniture::try_from(x)?)),
            RawRecord::GLOB(x) => Ok(Record::GlobalVariable(GlobalVariable::try_from(x)?)),
            RawRecord::GMST(x) => Ok(Record::GameSetting(GameSetting::try_from(x)?)),
            RawRecord::GRAS(x) => Ok(Record::Grass(Grass::try_from(x)?)),
            RawRecord::GRUP(x) => Ok(Record::Group(Group::try_from(x)?)),
            RawRecord::HAZD(x) => Ok(Record::Hazard(Hazard::try_from(x)?)),
            RawRecord::HDPT(x) => Ok(Record::HeadPart(HeadPart::try_from(x)?)),
            RawRecord::IDLE(x) => Ok(Record::IdleAnimation(IdleAnimation::try_from(x)?)),
            RawRecord::IDLM(x) => Ok(Record::IdleMarker(IdleMarker::try_from(x)?)),
            RawRecord::IMAD(x) => Ok(Record::ImageSpaceModifier(ImageSpaceModifier::try_from(x)?)),
            RawRecord::IMGS(x) => Ok(Record::ImageSpace(ImageSpace::try_from(x)?)),
            RawRecord::INFO(x) => Ok(Record::DialogueTopicInfo(DialogueTopicInfo::try_from(x)?)),
            RawRecord::INGR(x) => Ok(Record::Ingredient(Ingredient::try_from(x)?)),
            RawRecord::IPCT(x) => Ok(Record::ImpactData(ImpactData::try_from(x)?)),
            RawRecord::IPDS(x) => Ok(Record::ImpactDataSet(ImpactDataSet::try_from(x)?)),
            RawRecord::KEYM(x) => Ok(Record::Key(Key::try_from(x)?)),
            RawRecord::KYWD(x) => Ok(Record::Keyword(Keyword::try_from(x)?)),
            RawRecord::LAND(x) => Ok(Record::Landscape(Landscape::try_from(x)?)),
            RawRecord::LCRT(x) => Ok(Record::LocationRef(LocationRef::try_from(x)?)),
            RawRecord::LCTN(x) => Ok(Record::Location(Location::try_from(x)?)),
            RawRecord::LGTM(x) => Ok(Record::LightingTemplate(LightingTemplate::try_from(x)?)),
            RawRecord::LIGH(x) => Ok(Record::Light(Light::try_from(x)?)),
            RawRecord::LSCR(x) => Ok(Record::LoadScreen(LoadScreen::try_from(x)?)),
            RawRecord::LTEX(x) => Ok(Record::LandTexture(LandTexture::try_from(x)?)),
            RawRecord::LVLI(x) => Ok(Record::LeveledItem(LeveledItem::try_from(x)?)),
            RawRecord::LVLN(x) => Ok(Record::LeveledActor(LeveledActor::try_from(x)?)),
            RawRecord::LVSP(x) => Ok(Record::LeveledSpell(LeveledSpell::try_from(x)?)),
            RawRecord::MATO(x) => Ok(Record::MaterialObject(MaterialObject::try_from(x)?)),
            RawRecord::MATT(x) => Ok(Record::MaterialType(MaterialType::try_from(x)?)),
            RawRecord::MESG(x) => Ok(Record::Message(Message::try_from(x)?)),
            RawRecord::MGEF(x) => Ok(Record::MagicEffect(MagicEffect::try_from(x)?)),
            RawRecord::MISC(x) => Ok(Record::MiscItem(MiscItem::try_from(x)?)),
            RawRecord::MOVT(x) => Ok(Record::MovementType(MovementType::try_from(x)?)),
            RawRecord::MSTT(x) => Ok(Record::MovableStatic(MovableStatic::try_from(x)?)),
            RawRecord::MUSC(x) => Ok(Record::MusicType(MusicType::try_from(x)?)),
            RawRecord::MUST(x) => Ok(Record::MusicTrack(MusicTrack::try_from(x)?)),
            RawRecord::NAVI(x) => Ok(Record::Navigation(Navigation::try_from(x)?)),
            RawRecord::NAVM(x) => Ok(Record::NavMesh(NavMesh::try_from(x)?)),
            RawRecord::NOTE(x) => Ok(Record::Note(Note::try_from(x)?)),
            RawRecord::NPC_(x) => Ok(Record::NPC(NPC::try_from(x)?)),
            RawRecord::OTFT(x) => Ok(Record::Outfit(Outfit::try_from(x)?)),
            RawRecord::PACK(x) => Ok(Record::AIPackage(AIPackage::try_from(x)?)),
            RawRecord::PERK(x) => Ok(Record::Perk(Perk::try_from(x)?)),
            RawRecord::PGRE(x) => Ok(Record::Placedgrenade(Placedgrenade::try_from(x)?)),
            RawRecord::PHZD(x) => Ok(Record::Placedhazard(Placedhazard::try_from(x)?)),
            RawRecord::PROJ(x) => Ok(Record::Projectile(Projectile::try_from(x)?)),
            RawRecord::QUST(x) => Ok(Record::Quest(Quest::try_from(x)?)),
            RawRecord::RACE(x) => Ok(Record::Race(Race::try_from(x)?)),
            RawRecord::REFR(x) => Ok(Record::ObjectRef(ObjectRef::try_from(x)?)),
            RawRecord::REGN(x) => Ok(Record::Region(Region::try_from(x)?)),
            RawRecord::RELA(x) => Ok(Record::Relationship(Relationship::try_from(x)?)),
            RawRecord::REVB(x) => Ok(Record::ReverbParameters(ReverbParameters::try_from(x)?)),
            RawRecord::RFCT(x) => Ok(Record::VisualEffect(VisualEffect::try_from(x)?)),
            RawRecord::SCEN(x) => Ok(Record::Scene(Scene::try_from(x)?)),
            RawRecord::SCRL(x) => Ok(Record::Scroll(Scroll::try_from(x)?)),
            RawRecord::SHOU(x) => Ok(Record::Shout(Shout::try_from(x)?)),
            RawRecord::SLGM(x) => Ok(Record::SoulGem(SoulGem::try_from(x)?)),
            RawRecord::SMBN(x) => Ok(Record::StoryManagerBranchNode(
                StoryManagerBranchNode::try_from(x)?,
            )),
            RawRecord::SMEN(x) => Ok(Record::StoryManagerEventNode(
                StoryManagerEventNode::try_from(x)?,
            )),
            RawRecord::SMQN(x) => Ok(Record::StoryManagerQuestNode(
                StoryManagerQuestNode::try_from(x)?,
            )),
            RawRecord::SNCT(x) => Ok(Record::SoundCategory(SoundCategory::try_from(x)?)),
            RawRecord::SNDR(x) => Ok(Record::SoundReference(SoundReference::try_from(x)?)),
            RawRecord::SOPM(x) => Ok(Record::SoundOutputModel(SoundOutputModel::try_from(x)?)),
            RawRecord::SOUN(x) => Ok(Record::Sound(Sound::try_from(x)?)),
            RawRecord::SPEL(x) => Ok(Record::Spell(Spell::try_from(x)?)),
            RawRecord::SPGD(x) => Ok(Record::ShaderParticleGeometry(
                ShaderParticleGeometry::try_from(x)?,
            )),
            RawRecord::STAT(x) => Ok(Record::Static(Static::try_from(x)?)),
            RawRecord::TACT(x) => Ok(Record::TalkingActivator(TalkingActivator::try_from(x)?)),
            RawRecord::TREE(x) => Ok(Record::Tree(Tree::try_from(x)?)),
            RawRecord::TXST(x) => Ok(Record::TextureSet(TextureSet::try_from(x)?)),
            RawRecord::VTYP(x) => Ok(Record::VoiceType(VoiceType::try_from(x)?)),
            RawRecord::WATR(x) => Ok(Record::WaterType(WaterType::try_from(x)?)),
            RawRecord::WEAP(x) => Ok(Record::Weapon(Weapon::try_from(x)?)),
            RawRecord::WOOP(x) => Ok(Record::WordOfPower(WordOfPower::try_from(x)?)),
            RawRecord::WRLD(x) => Ok(Record::World(World::try_from(x)?)),
            RawRecord::WTHR(x) => Ok(Record::Weather(Weather::try_from(x)?)),
            RawRecord::UNKNOWN(x) => Ok(Record::Unknown(Unknown::try_from(x)?)),
        }
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Record::AIPackage(x) => write!(f, "{}", x),
            Record::AcousticSpace(x) => write!(f, "{}", x),
            Record::Action(x) => write!(f, "{}", x),
            Record::Activator(x) => write!(f, "{}", x),
            Record::ActorRef(x) => write!(f, "{}", x),
            Record::ActorValue(x) => write!(f, "{}", x),
            Record::AddonNode(x) => write!(f, "{}", x),
            Record::Alchemy(x) => write!(f, "{}", x),
            Record::Ammo(x) => write!(f, "{}", x),
            Record::AnimatedObjectInfo(x) => write!(f, "{}", x),
            Record::Apparatus(x) => write!(f, "{}", x),
            Record::Armor(x) => write!(f, "{}", x),
            Record::ArmorAddon(x) => write!(f, "{}", x),
            Record::ArtObject(x) => write!(f, "{}", x),
            Record::AssociationType(x) => write!(f, "{}", x),
            Record::BodyPartData(x) => write!(f, "{}", x),
            Record::Book(x) => write!(f, "{}", x),
            Record::CameraPath(x) => write!(f, "{}", x),
            Record::CameraShot(x) => write!(f, "{}", x),
            Record::Cell(x) => write!(f, "{}", x),
            Record::Class(x) => write!(f, "{}", x),
            Record::Climate(x) => write!(f, "{}", x),
            Record::CollisionLayer(x) => write!(f, "{}", x),
            Record::Color(x) => write!(f, "{}", x),
            Record::CombatStyle(x) => write!(f, "{}", x),
            Record::ConstructibleObj(x) => write!(f, "{}", x),
            Record::Container(x) => write!(f, "{}", x),
            Record::Debris(x) => write!(f, "{}", x),
            Record::DefaultObjectManager(x) => write!(f, "{}", x),
            Record::DialogueBranch(x) => write!(f, "{}", x),
            Record::DialogueTopic(x) => write!(f, "{}", x),
            Record::DialogueTopicInfo(x) => write!(f, "{}", x),
            Record::DialogueView(x) => write!(f, "{}", x),
            Record::Door(x) => write!(f, "{}", x),
            Record::DualCastArt(x) => write!(f, "{}", x),
            Record::EffectShader(x) => write!(f, "{}", x),
            Record::Enchantment(x) => write!(f, "{}", x),
            Record::EncounterZone(x) => write!(f, "{}", x),
            Record::EquipSlot(x) => write!(f, "{}", x),
            Record::Explosion(x) => write!(f, "{}", x),
            Record::Eyes(x) => write!(f, "{}", x),
            Record::Faction(x) => write!(f, "{}", x),
            Record::Flora(x) => write!(f, "{}", x),
            Record::Footstep(x) => write!(f, "{}", x),
            Record::FootstepSet(x) => write!(f, "{}", x),
            Record::FormList(x) => write!(f, "{}", x),
            Record::Furniture(x) => write!(f, "{}", x),
            Record::GameSetting(x) => write!(f, "{}", x),
            Record::GlobalVariable(x) => write!(f, "{}", x),
            Record::Grass(x) => write!(f, "{}", x),
            Record::Group(x) => write!(f, "{}", x),
            Record::Hazard(x) => write!(f, "{}", x),
            Record::HeadPart(x) => write!(f, "{}", x),
            Record::IdleAnimation(x) => write!(f, "{}", x),
            Record::IdleMarker(x) => write!(f, "{}", x),
            Record::ImageSpace(x) => write!(f, "{}", x),
            Record::ImageSpaceModifier(x) => write!(f, "{}", x),
            Record::ImpactData(x) => write!(f, "{}", x),
            Record::ImpactDataSet(x) => write!(f, "{}", x),
            Record::Ingredient(x) => write!(f, "{}", x),
            Record::Key(x) => write!(f, "{}", x),
            Record::Keyword(x) => write!(f, "{}", x),
            Record::LandTexture(x) => write!(f, "{}", x),
            Record::Landscape(x) => write!(f, "{}", x),
            Record::LeveledActor(x) => write!(f, "{}", x),
            Record::LeveledItem(x) => write!(f, "{}", x),
            Record::LeveledSpell(x) => write!(f, "{}", x),
            Record::Light(x) => write!(f, "{}", x),
            Record::LightingTemplate(x) => write!(f, "{}", x),
            Record::LoadScreen(x) => write!(f, "{}", x),
            Record::Location(x) => write!(f, "{}", x),
            Record::LocationRef(x) => write!(f, "{}", x),
            Record::MagicEffect(x) => write!(f, "{}", x),
            Record::MaterialObject(x) => write!(f, "{}", x),
            Record::MaterialType(x) => write!(f, "{}", x),
            Record::Message(x) => write!(f, "{}", x),
            Record::MiscItem(x) => write!(f, "{}", x),
            Record::MovableStatic(x) => write!(f, "{}", x),
            Record::MovementType(x) => write!(f, "{}", x),
            Record::MusicTrack(x) => write!(f, "{}", x),
            Record::MusicType(x) => write!(f, "{}", x),
            Record::NPC(x) => write!(f, "{}", x),
            Record::NavMesh(x) => write!(f, "{}", x),
            Record::Navigation(x) => write!(f, "{}", x),
            Record::Note(x) => write!(f, "{}", x),
            Record::ObjectRef(x) => write!(f, "{}", x),
            Record::Outfit(x) => write!(f, "{}", x),
            Record::Perk(x) => write!(f, "{}", x),
            Record::Placedgrenade(x) => write!(f, "{}", x),
            Record::Placedhazard(x) => write!(f, "{}", x),
            Record::Projectile(x) => write!(f, "{}", x),
            Record::Quest(x) => write!(f, "{}", x),
            Record::Race(x) => write!(f, "{}", x),
            Record::Region(x) => write!(f, "{}", x),
            Record::Relationship(x) => write!(f, "{}", x),
            Record::ReverbParameters(x) => write!(f, "{}", x),
            Record::Scene(x) => write!(f, "{}", x),
            Record::Scroll(x) => write!(f, "{}", x),
            Record::ShaderParticleGeometry(x) => write!(f, "{}", x),
            Record::Shout(x) => write!(f, "{}", x),
            Record::SoulGem(x) => write!(f, "{}", x),
            Record::Sound(x) => write!(f, "{}", x),
            Record::SoundCategory(x) => write!(f, "{}", x),
            Record::SoundOutputModel(x) => write!(f, "{}", x),
            Record::SoundReference(x) => write!(f, "{}", x),
            Record::Spell(x) => write!(f, "{}", x),
            Record::Static(x) => write!(f, "{}", x),
            Record::StoryManagerBranchNode(x) => write!(f, "{}", x),
            Record::StoryManagerEventNode(x) => write!(f, "{}", x),
            Record::StoryManagerQuestNode(x) => write!(f, "{}", x),
            Record::TalkingActivator(x) => write!(f, "{}", x),
            Record::TextureSet(x) => write!(f, "{}", x),
            Record::Tree(x) => write!(f, "{}", x),
            Record::VisualEffect(x) => write!(f, "{}", x),
            Record::VoiceType(x) => write!(f, "{}", x),
            Record::WaterType(x) => write!(f, "{}", x),
            Record::Weapon(x) => write!(f, "{}", x),
            Record::Weather(x) => write!(f, "{}", x),
            Record::WordOfPower(x) => write!(f, "{}", x),
            Record::World(x) => write!(f, "{}", x),
            Record::Unknown(x) => write!(f, "{}", x),
        }
    }
}

impl Record {
    pub fn localize(&mut self, string_table: &StringTables) {
        match self {
            Record::Alchemy(x) => x.localize(string_table),
            Record::Book(x) => x.localize(string_table),
            Record::Class(x) => x.localize(string_table),
            Record::Color(x) => x.localize(string_table),
            Record::Group(x) => x.localize(string_table),
            Record::GameSetting(x) => x.localize(string_table),
            _ => {}
        }
    }

    pub fn magic(&self) -> [u8; 4] {
        match self {
            Record::Action(_) => *b"AACT",
            Record::ActorRef(_) => *b"ACHR",
            Record::Activator(_) => *b"ACTI",
            Record::AddonNode(_) => *b"ADDN",
            Record::Alchemy(_) => *b"ALCH",
            Record::Ammo(_) => *b"AMMO",
            Record::AnimatedObjectInfo(_) => *b"ANIO",
            Record::Apparatus(_) => *b"APPA",
            Record::ArmorAddon(_) => *b"ARMA",
            Record::Armor(_) => *b"ARMO",
            Record::ArtObject(_) => *b"ARTO",
            Record::AcousticSpace(_) => *b"ASPC",
            Record::AssociationType(_) => *b"ASTP",
            Record::ActorValue(_) => *b"AVIF",
            Record::Book(_) => *b"BOOK",
            Record::BodyPartData(_) => *b"BPTD",
            Record::CameraShot(_) => *b"CAMS",
            Record::Cell(_) => *b"CELL",
            Record::Class(_) => *b"CLAS",
            Record::Color(_) => *b"CLFM",
            Record::Climate(_) => *b"CLMT",
            Record::ConstructibleObj(_) => *b"COBJ",
            Record::CollisionLayer(_) => *b"COLL",
            Record::Container(_) => *b"CONT",
            Record::CameraPath(_) => *b"CPTH",
            Record::CombatStyle(_) => *b"CSTY",
            Record::Debris(_) => *b"DEBR",
            Record::DialogueTopic(_) => *b"DIAL",
            Record::DialogueBranch(_) => *b"DLBR",
            Record::DialogueView(_) => *b"DLVW",
            Record::DefaultObjectManager(_) => *b"DOBJ",
            Record::Door(_) => *b"DOOR",
            Record::DualCastArt(_) => *b"DUAL",
            Record::EncounterZone(_) => *b"ECZN",
            Record::EffectShader(_) => *b"EFSH",
            Record::Enchantment(_) => *b"ENCH",
            Record::EquipSlot(_) => *b"EQUP",
            Record::Explosion(_) => *b"EXPL",
            Record::Eyes(_) => *b"EYES",
            Record::Faction(_) => *b"FACT",
            Record::Flora(_) => *b"FLOR",
            Record::FormList(_) => *b"FLST",
            Record::Footstep(_) => *b"FSTP",
            Record::FootstepSet(_) => *b"FSTS",
            Record::Furniture(_) => *b"FURN",
            Record::GlobalVariable(_) => *b"GLOB",
            Record::GameSetting(_) => *b"GMST",
            Record::Grass(_) => *b"GRAS",
            Record::Group(_) => *b"GRUP",
            Record::Hazard(_) => *b"HAZD",
            Record::HeadPart(_) => *b"HDPT",
            Record::IdleAnimation(_) => *b"IDLE",
            Record::IdleMarker(_) => *b"IDLM",
            Record::ImageSpaceModifier(_) => *b"IMAD",
            Record::ImageSpace(_) => *b"IMGS",
            Record::DialogueTopicInfo(_) => *b"INFO",
            Record::Ingredient(_) => *b"INGR",
            Record::ImpactData(_) => *b"IPCT",
            Record::ImpactDataSet(_) => *b"IPDS",
            Record::Key(_) => *b"KEYM",
            Record::Keyword(_) => *b"KYWD",
            Record::Landscape(_) => *b"LAND",
            Record::LocationRef(_) => *b"LCRT",
            Record::Location(_) => *b"LCTN",
            Record::LightingTemplate(_) => *b"LGTM",
            Record::Light(_) => *b"LIGH",
            Record::LoadScreen(_) => *b"LSCR",
            Record::LandTexture(_) => *b"LTEX",
            Record::LeveledItem(_) => *b"LVLI",
            Record::LeveledActor(_) => *b"LVLN",
            Record::LeveledSpell(_) => *b"LVSP",
            Record::MaterialObject(_) => *b"MATO",
            Record::MaterialType(_) => *b"MATT",
            Record::Message(_) => *b"MESG",
            Record::MagicEffect(_) => *b"MGEF",
            Record::MiscItem(_) => *b"MISC",
            Record::MovementType(_) => *b"MOVT",
            Record::MovableStatic(_) => *b"MSTT",
            Record::MusicType(_) => *b"MUSC",
            Record::MusicTrack(_) => *b"MUST",
            Record::Navigation(_) => *b"NAVI",
            Record::NavMesh(_) => *b"NAVM",
            Record::Note(_) => *b"NOTE",
            Record::NPC(_) => *b"NPC_",
            Record::Outfit(_) => *b"OTFT",
            Record::AIPackage(_) => *b"PACK",
            Record::Perk(_) => *b"PERK",
            Record::Placedgrenade(_) => *b"PGRE",
            Record::Placedhazard(_) => *b"PHZD",
            Record::Projectile(_) => *b"PROJ",
            Record::Quest(_) => *b"QUST",
            Record::Race(_) => *b"RACE",
            Record::ObjectRef(_) => *b"REFR",
            Record::Region(_) => *b"REGN",
            Record::Relationship(_) => *b"RELA",
            Record::ReverbParameters(_) => *b"REVB",
            Record::VisualEffect(_) => *b"RFCT",
            Record::Scene(_) => *b"SCEN",
            Record::Scroll(_) => *b"SCRL",
            Record::Shout(_) => *b"SHOU",
            Record::SoulGem(_) => *b"SLGM",
            Record::StoryManagerBranchNode(_) => *b"SMBN",
            Record::StoryManagerEventNode(_) => *b"SMEN",
            Record::StoryManagerQuestNode(_) => *b"SMQN",
            Record::SoundCategory(_) => *b"SNCT",
            Record::SoundReference(_) => *b"SNDR",
            Record::SoundOutputModel(_) => *b"SOPM",
            Record::Sound(_) => *b"SOUN",
            Record::Spell(_) => *b"SPEL",
            Record::ShaderParticleGeometry(_) => *b"SPGD",
            Record::Static(_) => *b"STAT",
            Record::TalkingActivator(_) => *b"TACT",
            Record::Tree(_) => *b"TREE",
            Record::TextureSet(_) => *b"TXST",
            Record::VoiceType(_) => *b"VTYP",
            Record::WaterType(_) => *b"WATR",
            Record::Weapon(_) => *b"WEAP",
            Record::WordOfPower(_) => *b"WOOP",
            Record::World(_) => *b"WRLD",
            Record::Weather(_) => *b"WTHR",
            Record::Unknown(_) => *b"UNKN",
        }
    }

    pub fn form_id(&self) -> Option<u32> {
        match self {
            Record::Action(rec) => Some(rec.header.form_id),
            Record::ActorRef(rec) => Some(rec.header.form_id),
            Record::Activator(rec) => Some(rec.header.form_id),
            Record::AddonNode(rec) => Some(rec.header.form_id),
            Record::Alchemy(rec) => Some(rec.header.form_id),
            Record::Ammo(rec) => Some(rec.header.form_id),
            Record::AnimatedObjectInfo(rec) => Some(rec.header.form_id),
            Record::Apparatus(rec) => Some(rec.header.form_id),
            Record::ArmorAddon(rec) => Some(rec.header.form_id),
            Record::Armor(rec) => Some(rec.header.form_id),
            Record::ArtObject(rec) => Some(rec.header.form_id),
            Record::AcousticSpace(rec) => Some(rec.header.form_id),
            Record::AssociationType(rec) => Some(rec.header.form_id),
            Record::ActorValue(rec) => Some(rec.header.form_id),
            Record::Book(rec) => Some(rec.header.form_id),
            Record::BodyPartData(rec) => Some(rec.header.form_id),
            Record::CameraShot(rec) => Some(rec.header.form_id),
            Record::Cell(rec) => Some(rec.header.form_id),
            Record::Class(rec) => Some(rec.header.form_id),
            Record::Color(rec) => Some(rec.header.form_id),
            Record::Climate(rec) => Some(rec.header.form_id),
            Record::ConstructibleObj(rec) => Some(rec.header.form_id),
            Record::CollisionLayer(rec) => Some(rec.header.form_id),
            Record::Container(rec) => Some(rec.header.form_id),
            Record::CameraPath(rec) => Some(rec.header.form_id),
            Record::CombatStyle(rec) => Some(rec.header.form_id),
            Record::Debris(rec) => Some(rec.header.form_id),
            Record::DialogueTopic(rec) => Some(rec.header.form_id),
            Record::DialogueBranch(rec) => Some(rec.header.form_id),
            Record::DialogueView(rec) => Some(rec.header.form_id),
            Record::DefaultObjectManager(rec) => Some(rec.header.form_id),
            Record::Door(rec) => Some(rec.header.form_id),
            Record::DualCastArt(rec) => Some(rec.header.form_id),
            Record::EncounterZone(rec) => Some(rec.header.form_id),
            Record::EffectShader(rec) => Some(rec.header.form_id),
            Record::Enchantment(rec) => Some(rec.header.form_id),
            Record::EquipSlot(rec) => Some(rec.header.form_id),
            Record::Explosion(rec) => Some(rec.header.form_id),
            Record::Eyes(rec) => Some(rec.header.form_id),
            Record::Faction(rec) => Some(rec.header.form_id),
            Record::Flora(rec) => Some(rec.header.form_id),
            Record::FormList(rec) => Some(rec.header.form_id),
            Record::Footstep(rec) => Some(rec.header.form_id),
            Record::FootstepSet(rec) => Some(rec.header.form_id),
            Record::Furniture(rec) => Some(rec.header.form_id),
            Record::GlobalVariable(rec) => Some(rec.header.form_id),
            Record::GameSetting(rec) => Some(rec.header.form_id),
            Record::Grass(rec) => Some(rec.header.form_id),
            Record::Group(_) => None,
            Record::Hazard(rec) => Some(rec.header.form_id),
            Record::HeadPart(rec) => Some(rec.header.form_id),
            Record::IdleAnimation(rec) => Some(rec.header.form_id),
            Record::IdleMarker(rec) => Some(rec.header.form_id),
            Record::ImageSpaceModifier(rec) => Some(rec.header.form_id),
            Record::ImageSpace(rec) => Some(rec.header.form_id),
            Record::DialogueTopicInfo(rec) => Some(rec.header.form_id),
            Record::Ingredient(rec) => Some(rec.header.form_id),
            Record::ImpactData(rec) => Some(rec.header.form_id),
            Record::ImpactDataSet(rec) => Some(rec.header.form_id),
            Record::Key(rec) => Some(rec.header.form_id),
            Record::Keyword(rec) => Some(rec.header.form_id),
            Record::Landscape(rec) => Some(rec.header.form_id),
            Record::LocationRef(rec) => Some(rec.header.form_id),
            Record::Location(rec) => Some(rec.header.form_id),
            Record::LightingTemplate(rec) => Some(rec.header.form_id),
            Record::Light(rec) => Some(rec.header.form_id),
            Record::LoadScreen(rec) => Some(rec.header.form_id),
            Record::LandTexture(rec) => Some(rec.header.form_id),
            Record::LeveledItem(rec) => Some(rec.header.form_id),
            Record::LeveledActor(rec) => Some(rec.header.form_id),
            Record::LeveledSpell(rec) => Some(rec.header.form_id),
            Record::MaterialObject(rec) => Some(rec.header.form_id),
            Record::MaterialType(rec) => Some(rec.header.form_id),
            Record::Message(rec) => Some(rec.header.form_id),
            Record::MagicEffect(rec) => Some(rec.header.form_id),
            Record::MiscItem(rec) => Some(rec.header.form_id),
            Record::MovementType(rec) => Some(rec.header.form_id),
            Record::MovableStatic(rec) => Some(rec.header.form_id),
            Record::MusicType(rec) => Some(rec.header.form_id),
            Record::MusicTrack(rec) => Some(rec.header.form_id),
            Record::Navigation(rec) => Some(rec.header.form_id),
            Record::NavMesh(rec) => Some(rec.header.form_id),
            Record::Note(rec) => Some(rec.header.form_id),
            Record::NPC(rec) => Some(rec.header.form_id),
            Record::Outfit(rec) => Some(rec.header.form_id),
            Record::AIPackage(rec) => Some(rec.header.form_id),
            Record::Perk(rec) => Some(rec.header.form_id),
            Record::Placedgrenade(rec) => Some(rec.header.form_id),
            Record::Placedhazard(rec) => Some(rec.header.form_id),
            Record::Projectile(rec) => Some(rec.header.form_id),
            Record::Quest(rec) => Some(rec.header.form_id),
            Record::Race(rec) => Some(rec.header.form_id),
            Record::ObjectRef(rec) => Some(rec.header.form_id),
            Record::Region(rec) => Some(rec.header.form_id),
            Record::Relationship(rec) => Some(rec.header.form_id),
            Record::ReverbParameters(rec) => Some(rec.header.form_id),
            Record::VisualEffect(rec) => Some(rec.header.form_id),
            Record::Scene(rec) => Some(rec.header.form_id),
            Record::Scroll(rec) => Some(rec.header.form_id),
            Record::Shout(rec) => Some(rec.header.form_id),
            Record::SoulGem(rec) => Some(rec.header.form_id),
            Record::StoryManagerBranchNode(rec) => Some(rec.header.form_id),
            Record::StoryManagerEventNode(rec) => Some(rec.header.form_id),
            Record::StoryManagerQuestNode(rec) => Some(rec.header.form_id),
            Record::SoundCategory(rec) => Some(rec.header.form_id),
            Record::SoundReference(rec) => Some(rec.header.form_id),
            Record::SoundOutputModel(rec) => Some(rec.header.form_id),
            Record::Sound(rec) => Some(rec.header.form_id),
            Record::Spell(rec) => Some(rec.header.form_id),
            Record::ShaderParticleGeometry(rec) => Some(rec.header.form_id),
            Record::Static(rec) => Some(rec.header.form_id),
            Record::TalkingActivator(rec) => Some(rec.header.form_id),
            Record::Tree(rec) => Some(rec.header.form_id),
            Record::TextureSet(rec) => Some(rec.header.form_id),
            Record::VoiceType(rec) => Some(rec.header.form_id),
            Record::WaterType(rec) => Some(rec.header.form_id),
            Record::Weapon(rec) => Some(rec.header.form_id),
            Record::WordOfPower(rec) => Some(rec.header.form_id),
            Record::World(rec) => Some(rec.header.form_id),
            Record::Weather(rec) => Some(rec.header.form_id),
            Record::Unknown(rec) => Some(rec.header.form_id),
        }
    }
}

fn get_cursor(data: &[u8], compressed: bool) -> Vec<u8> {
    if compressed {
        let mut cursor = Cursor::new(&data[4..]);
        let mut d = ZlibDecoder::new(&mut cursor);
        let mut decomp = Vec::new();
        d.read_to_end(&mut decomp).unwrap();
        decomp
    } else {
        data.into()
    }
}
