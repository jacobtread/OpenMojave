use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID, LocalizedString};
use crate::error::Error;
use crate::fields::{
    AlternateTextures, BodyTemplate, BodyTemplate2, DestructionData, ObjectBounds, ScriptList,
    Textures, BAMT, BIDS, BMCT, BOD2, BODT, DATA, DESC, DNAM, EAMT, EDID, EITM, ETYP, FULL, ICO2,
    ICON, KSIZ, KWDA, MIC2, MICO, MO2S, MO2T, MO4S, MO4T, MOD2, MOD4, MODL, MODS, MODT, OBND, RNAM,
    TNAM, VMAD, YNAM, ZNAM,
};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"ARMO")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ARMO {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,

    #[br(calc(localized))]
    #[bw(ignore)]
    pub localized: bool,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmorData {
    pub value: u32,
    pub weight: f32,
}

impl TryFrom<DATA> for ArmorData {
    type Error = Error;

    fn try_from(raw: DATA) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Armor {
    pub header: RecordHeader,
    pub edid: String,
    pub scripts: Option<ScriptList>,
    pub bounds: Option<ObjectBounds>,
    pub full_name: Option<LocalizedString>,
    pub enchantment: Option<FormID>,
    pub enchantment_amount: Option<u16>,
    pub model_filename: Option<String>,
    pub model_textures: Option<Textures>,
    pub alternate_textures: Option<AlternateTextures>,
    pub male_model_filename: Option<String>,
    pub male_model_textures: Option<Textures>,
    pub male_alternate_textures: Option<AlternateTextures>,
    pub male_inventory_image: Option<String>,
    pub male_message_image: Option<String>,
    pub female_model_filename: Option<String>,
    pub female_model_textures: Option<Textures>,
    pub female_alternate_textures: Option<AlternateTextures>,
    pub female_inventory_image: Option<String>,
    pub female_message_image: Option<String>,
    pub body_template: Option<BodyTemplate>,
    pub body_template_2: Option<BodyTemplate2>,
    pub destruction_data: Option<DestructionData>,
    pub pickup_sound: Option<FormID>,
    pub drop_sound: Option<FormID>,
    pub ragdoll: Option<String>,
    pub equip_slot: Option<FormID>,
    pub bash_impact_data_set: Option<FormID>,
    pub bash_material: Option<FormID>,
    pub rnam: FormID,
    pub keywords: Vec<FormID>,
    pub description: Option<LocalizedString>,
    pub armatures: Vec<FormID>,
    pub data: ArmorData,
    pub armor_rating: u32,
    pub template: Option<FormID>,
}

impl fmt::Display for Armor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Armor ({})", self.edid)
    }
}

impl TryFrom<ARMO> for Armor {
    type Error = Error;

    fn try_from(raw: ARMO) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let scripts = VMAD::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let bounds = OBND::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let full_name = match (FULL::read(&mut cursor), raw.localized) {
            (Ok(f), true) => Some(LocalizedString::Localized(f.try_into()?)),
            (Ok(z), false) => Some(LocalizedString::ZString(z.try_into()?)),
            (Err(_), _) => None,
        };
        let enchantment = EITM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let enchantment_amount = EAMT::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let model_filename = MODL::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let model_textures = MODT::read(&mut cursor)
            .ok()
            .map(|modt| Textures::load(modt, raw.header.internal_version))
            .transpose()?;
        let alternate_textures = MODS::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let male_model_filename = MOD2::read(&mut cursor)
            .ok()
            .map(|m| MODL::from(m).try_into())
            .transpose()?;
        let male_model_textures = MO2T::read(&mut cursor)
            .ok()
            .map(|modt| Textures::load(modt.into(), raw.header.internal_version))
            .transpose()?;
        let male_alternate_textures = MO2S::read(&mut cursor)
            .ok()
            .map(|m| Into::<MODS>::into(m).try_into())
            .transpose()?;
        let male_inventory_image = ICON::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let male_message_image = MICO::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let female_model_filename = MOD4::read(&mut cursor)
            .ok()
            .map(|m| Into::<MODL>::into(m).try_into())
            .transpose()?;
        let female_model_textures = MO4T::read(&mut cursor)
            .ok()
            .map(|modt| Textures::load(modt.into(), raw.header.internal_version))
            .transpose()?;
        let female_alternate_textures = MO4S::read(&mut cursor)
            .ok()
            .map(|m| Into::<MODS>::into(m).try_into())
            .transpose()?;
        let female_inventory_image = ICO2::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let female_message_image = MIC2::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let body_template = BODT::read(&mut cursor)
            .ok()
            .map(|bodt| BodyTemplate::load(bodt, raw.header.internal_version))
            .transpose()?;
        let body_template_2 = BOD2::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let destruction_data = DestructionData::load(&mut cursor)?;
        let pickup_sound = YNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let drop_sound = ZNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let ragdoll = BMCT::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let equip_slot = ETYP::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let bash_impact_data_set = BIDS::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let bash_material = BAMT::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let rnam = RNAM::read(&mut cursor)?.try_into()?;
        let keyword_count: Option<u32> = KSIZ::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let mut keywords = Vec::new();
        if let Some(kc) = keyword_count {
            for _ in 0..kc {
                // It's actually only up to keyword count
                if let Ok(kwda) = KWDA::read(&mut cursor) {
                    keywords.push(FormID::read_le(&mut Cursor::new(kwda.data)).unwrap());
                }
            }
        }
        let description = match (DESC::read(&mut cursor), raw.localized) {
            (Ok(f), true) => Some(LocalizedString::Localized(f.try_into()?)),
            (Ok(z), false) => Some(LocalizedString::ZString(z.try_into()?)),
            (Err(_), _) => None,
        };
        let mut armatures = Vec::new();
        while let Ok(modl) = MODL::read(&mut cursor) {
            armatures.push(modl.try_into()?);
        }
        let data = DATA::read(&mut cursor)?.try_into()?;
        let armor_rating = DNAM::read(&mut cursor)?.try_into()?;
        let template = TNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            scripts,
            bounds,
            full_name,
            enchantment,
            enchantment_amount,
            model_filename,
            model_textures,
            alternate_textures,
            male_model_filename,
            male_model_textures,
            male_alternate_textures,
            male_inventory_image,
            male_message_image,
            female_model_filename,
            female_model_textures,
            female_alternate_textures,
            female_inventory_image,
            female_message_image,
            body_template,
            body_template_2,
            destruction_data,
            pickup_sound,
            drop_sound,
            ragdoll,
            equip_slot,
            bash_impact_data_set,
            bash_material,
            rnam,
            keywords,
            description,
            armatures,
            data,
            armor_rating,
            template,
        })
    }
}
