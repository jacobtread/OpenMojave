use crate::esp::{
    record::{
        sub::{CNAM, DATA, DELE, HEDR, MAST, OFST, ONAM, SNAM},
        Collection, FromSubRecord, Record, RecordParseError, RecordParser, RecordType,
    },
    shared::FormId,
};
use nom::{
    combinator::map,
    number::complete::{le_f32, le_u32},
    sequence::tuple,
    IResult,
};

/// Plugin Info
#[derive(Debug)]
pub struct TES4 {
    // Contains additional details about the plugin
    pub hedr: HEDR,
    pub author: String,
    pub description: Option<String>,
    pub masters: Vec<String>,
    pub form_overrides: Option<Vec<FormId>>,
}

impl Record for TES4 {
    const TYPE: RecordType = RecordType::from_value(b"TES4");

    fn parse<'b>(parser: &mut RecordParser<'_, 'b>) -> Result<Self, RecordParseError<'b>> {
        let hedr = parser.parse::<HEDR>(HEDR)?;

        parser.skip_type(OFST);
        parser.skip_type(DELE);

        let author: String = parser.parse::<String>(CNAM)?;
        let description = parser.try_parse::<String>(SNAM)?;

        let mut masters: Vec<String> = Vec::new();

        // Consume master data collection
        while let Some(mast) = parser.try_parse::<String>(MAST)? {
            // Data can be ignored as its not used (It's usually 0)
            parser.skip_type(DATA);
            masters.push(mast);
        }

        let form_overrides = parser
            .try_parse::<Collection<FormId>>(ONAM)?
            .map(Collection::into_inner);

        parser.skip_type(RecordType::from_value(b"DELE"));

        Ok(Self {
            hedr,
            author,
            description,
            masters,
            form_overrides,
        })
    }
}

/// "HEDR" sub record
#[derive(Debug)]
pub struct HEDR {
    /// 0.94 in most files; 1.7 in recent versions of Update.esm.
    pub version: f32,
    /// Number of records and groups (not including TES4 record itself).
    pub num_records: u32,
    /// Next available object ID.
    pub next_object_id: u32,
}

impl FromSubRecord for HEDR {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((le_f32, le_u32, le_u32)),
            |(version, num_records, next_object_id)| HEDR {
                version,
                num_records,
                next_object_id,
            },
        )(input)
    }
}
