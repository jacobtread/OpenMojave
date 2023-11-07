use super::{
    armo::{ArmorData, DNAMFlags, BMDT},
    prelude::{equipment_type::EquipmentType, model::ModelData, object_bounds::ObjectBounds, *},
};

/// Armor Addon
#[derive(Debug)]
pub struct ARMA {
    pub editor_id: EditorId,
    pub object_bounds: ObjectBounds,
    pub name: Option<String>,
    pub biped_data: BMDT,
    pub male_biped_model_data: ModelData,
    pub male_world_model_data: ModelData,
    pub male_inventory_icon_file_name: Option<String>,
    pub male_message_icon_file_name: Option<String>,
    pub female_biped_model_data: ModelData,
    pub female_world_model_data: ModelData,
    pub female_inventory_icon_file_name: Option<String>,
    pub female_message_icon_file_name: Option<String>,
    pub equipment_type: EquipmentType,
    pub data: ArmorData,
    pub dnam: DNAM,
}

impl Record for ARMA {
    const TYPE: RecordType = RecordType::new(b"ARMA");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let editor_id: EditorId = parser.parse(EDID)?;
        let object_bounds: ObjectBounds = parser.parse(OBND)?;
        let name: Option<String> = parser.try_parse(FULL)?;
        let biped_data: BMDT = parser.parse(BMDT)?;
        let male_biped_model_data: ModelData = ModelData::parse_first(parser)?
            .ok_or_else(|| RecordParseError::Custom("Missing male_biped_model_data".to_string()))?;
        let male_world_model_data: ModelData = ModelData::parse_second(parser)?
            .ok_or_else(|| RecordParseError::Custom("Missing male_world_model_data".to_string()))?;

        let male_inventory_icon_file_name: Option<String> = parser.try_parse(ICON)?;
        let male_message_icon_file_name: Option<String> = parser.try_parse(MICO)?;

        let female_biped_model_data: ModelData = ModelData::parse_third(parser)?
            .ok_or_else(|| RecordParseError::Custom("Missing male_biped_model_data".to_string()))?;
        let female_world_model_data: ModelData = ModelData::parse_fourth(parser)?
            .ok_or_else(|| RecordParseError::Custom("Missing male_world_model_data".to_string()))?;

        let female_inventory_icon_file_name: Option<String> = parser.try_parse(ICO2)?;
        let female_message_icon_file_name: Option<String> = parser.try_parse(MIC2)?;
        let equipment_type: EquipmentType = parser.parse(ETYP)?;
        let data: ArmorData = parser.parse(DATA)?;
        let dnam: DNAM = parser.parse(DNAM)?;

        Ok(Self {
            editor_id,
            object_bounds,
            name,
            biped_data,
            male_biped_model_data,
            male_world_model_data,
            male_inventory_icon_file_name,
            male_message_icon_file_name,
            female_biped_model_data,
            female_world_model_data,
            female_inventory_icon_file_name,
            female_message_icon_file_name,
            equipment_type,
            data,
            dnam,
        })
    }
}

#[derive(Debug)]
pub struct DNAM {
    pub ar: i16,
    pub flags: DNAMFlags,
}

impl FromRecordBytes for DNAM {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(tuple((le_i16, DNAMFlags::parse, rest)), |(ar, flags, _)| {
            Self { ar, flags }
        })(input)
    }
}
