use std::{
    any::{Any, TypeId},
    sync::Arc,
};

use bevy::utils::HashMap;
use parking_lot::Mutex;

use super::record::{
    records::{
        acti::ACTI, alch::ALCH, arma::ARMA, armo::ARMO, book::BOOK, bptd::BPTD, cell::CELL,
        clas::CLAS, cont::CONT, crea::CREA, dial::DIAL, door::DOOR, ench::ENCH, fact::FACT,
        glob::GLOB, gmst::GMST, ingr::INGR, land::LAND, ligh::LIGH, ltex::LTEX, lvlc::LVLC,
        lvli::LVLI, mgef::MGEF, misc::MISC, npc::NPC_, prelude::FormId, race::RACE, regn::REGN,
        scpt::SCPT, soun::SOUN, spel::SPEL, stat::STAT, weap::WEAP, RecordValue,
    },
    FromRecordBytes, Group, RawEsmEntry, Record, RecordType,
};
use bevy::log::warn;

pub struct EsmStore {
    activators: Store<ACTI>,
    ingestibles: Store<ALCH>,
    armors: Store<ARMO>,
    body_parts: Store<BPTD>,
    books: Store<BOOK>,
    classes: Store<CLAS>,
    containers: Store<CONT>,
    creatures: Store<CREA>,
    dialogue: Store<DIAL>,
    doors: Store<DOOR>,
    enchants: Store<ENCH>,
    factions: Store<FACT>,
    globals: Store<GLOB>,
    ingredients: Store<INGR>,
    creature_lists: Store<LVLC>,
    item_lists: Store<LVLI>,
    lights: Store<LIGH>,
    misc_items: Store<MISC>,
    npcs: Store<NPC_>,
    races: Store<RACE>,
    regions: Store<REGN>,
    sounds: Store<SOUN>,
    spells: Store<SPEL>,
    statics: Store<STAT>,
    weapons: Store<WEAP>,
    game_settings: Store<GMST>,
    scripts: Store<SCPT>,

    cells: Store<CELL>,
    lands: Store<LAND>,
    land_textures: Store<LTEX>,
    magic_effects: Store<MGEF>,
}

#[derive(Clone)]
pub struct Store<R: Record> {
    inner: Arc<StoreInner<R>>,
}

/// Inner portion of the store thats behind an arc
/// so it can be cheaply cloned
pub struct StoreInner<R: Record> {
    // Store for static values loaded from files
    values_static: HashMap<String, R>,
    /// Stores for dynamic values created at runtime
    values_dynamic: Mutex<HashMap<String, R>>,
}
