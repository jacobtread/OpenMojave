use super::{
    collection::script::Script,
    record::{Record, RecordParser, RecordType},
    sub::{
        edid::EDID, inam::INAM, name::NAME, xezn::XEZN, xppa::XPPA, xprd::XPRD, xrgb::XRGB,
        xrgd::XRGD,
    },
};

pub struct ACHR {
    pub editor_id: EDID,
    pub base: NAME,
    pub encounter_zone: Option<XEZN>,
    pub ragdoll_data: Option<XRGD>,
    pub ragdoll_biped_data: Option<XRGB>,
    pub idle_time: XPRD,
    pub patrol_script_marker: XPPA,
    pub idle: INAM,
    pub embedded_script: Script,
}

impl Record for ACHR {
    const TYPE: RecordType = RecordType::from_value(b"ACHR");

    fn parse<'b>(
        parser: &mut RecordParser<'_, 'b>,
    ) -> Result<Self, super::record::RecordParseError<'b>> {
        todo!()
    }
}
