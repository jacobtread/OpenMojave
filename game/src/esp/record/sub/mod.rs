use nom::combinator::{map, rest};
use nom::IResult;

use super::RecordType;

pub const HEDR: RecordType = RecordType::from_value(b"HEDR");
pub const MAST: RecordType = RecordType::from_value(b"MAST");
pub const NAME: RecordType = RecordType::from_value(b"NAME");
pub const CNAM: RecordType = RecordType::from_value(b"CNAM");
pub const SNAM: RecordType = RecordType::from_value(b"SNAM");
pub const ONAM: RecordType = RecordType::from_value(b"ONAM");
pub const EDID: RecordType = RecordType::from_value(b"EDID");
pub const DATA: RecordType = RecordType::from_value(b"DATA");
pub const XEZN: RecordType = RecordType::from_value(b"XEZN");
