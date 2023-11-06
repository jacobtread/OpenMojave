use super::{fact::FACT, mset::MSET, prelude::*};

/// Media Relation Controller
#[derive(Debug)]
pub struct ALOC {
    pub editor_id: EditorId,
    pub name: Option<String>,
    pub location_delay: Option<f32>,
    pub day_start: Option<u32>,
    pub night_start: Option<u32>,
    pub retrigger_delay: Option<f32>,
    pub nuetral_media_set: Vec<TypedFormId<MSET>>,
    pub ally_media_set: Vec<TypedFormId<MSET>>,
    pub friend_media_set: Vec<TypedFormId<MSET>>,
    pub enemy_media_set: Vec<TypedFormId<MSET>>,
    pub location_media_set: Vec<TypedFormId<MSET>>,
    pub battle_media_set: Vec<TypedFormId<MSET>>,
    pub conditional_faction: Option<TypedFormId<FACT>>,
}

impl Record for ALOC {
    const TYPE: RecordType = RecordType::new(b"ALOC");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let name: Option<String> = parser.try_parse(FULL)?;

        // Possibly a combination of flags and enums
        parser.skip_type(NAM1);
        parser.skip_type(NAM2);
        parser.skip_type(NAM3);

        let location_delay: Option<f32> = parser.try_parse(NAM4)?;
        let day_start: Option<u32> = parser.try_parse(NAM5)?;
        let night_start: Option<u32> = parser.try_parse(NAM6)?;
        let retrigger_delay: Option<f32> = parser.try_parse(NAM7)?;
        let nuetral_media_set: Vec<TypedFormId<MSET>> = parser.try_parse_many(HNAM)?;
        let ally_media_set: Vec<TypedFormId<MSET>> = parser.try_parse_many(ZNAM)?;
        let friend_media_set: Vec<TypedFormId<MSET>> = parser.try_parse_many(XNAM)?;
        let enemy_media_set: Vec<TypedFormId<MSET>> = parser.try_parse_many(YNAM)?;
        let location_media_set: Vec<TypedFormId<MSET>> = parser.try_parse_many(LNAM)?;
        let battle_media_set: Vec<TypedFormId<MSET>> = parser.try_parse_many(GNAM)?;
        let conditional_faction: Option<TypedFormId<FACT>> = parser.try_parse(RNAM)?;

        // ??
        parser.skip_type(FNAM);

        Ok(Self {
            editor_id,
            name,
            location_delay,
            day_start,
            night_start,
            retrigger_delay,
            nuetral_media_set,
            ally_media_set,
            friend_media_set,
            enemy_media_set,
            location_media_set,
            battle_media_set,
            conditional_faction,
        })
    }
}
