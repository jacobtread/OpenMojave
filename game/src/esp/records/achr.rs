// use serde_ini::parse;

// use crate::esp::{
//     records::sub::{edid::EDID, name::NAME, xezn::XEZN},
//     shared::{EditorId, FormId},
// };

// use super::{
//     collection::script::Script,
//     record::{Record, RecordParser, RecordType},
// };

// pub struct ACHR {
//     pub editor_id: EditorId,
//     pub base: FormId,
//     pub encounter_zone: Option<FormId>,
//     pub ragdoll_data: Option<XRGD>,
//     pub ragdoll_biped_data: Option<XRGB>,
//     pub idle_time: XPRD,
//     pub patrol_script_marker: XPPA,
//     pub idle: INAM,
//     pub embedded_script: Script,
// }

// impl Record for ACHR {
//     const TYPE: RecordType = RecordType::from_value(b"ACHR");

//     fn parse<'b>(
//         parser: &mut RecordParser<'_, 'b>,
//     ) -> Result<Self, super::record::RecordParseError<'b>> {
//         let editor_id = parser.parse(EDID::TYPE, EDID::parse)?;
//         let base = parser.parse(NAME::TYPE, NAME::parse)?;
//         let encounter_zone = parser.try_parse(XEZN::TYPE, XEZN::parse)?;

//         todo!()
//     }
// }
