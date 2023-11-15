use std::any::{Any, TypeId};

use bevy::utils::HashMap;

use super::record::{
    records::{prelude::FormId, RecordValue},
    FromRecordBytes, Group, RawEsmEntry,
};
use bevy::log::warn;

pub struct GlobalRecordStore {
    records: HashMap<FormId, RecordValue>,
    groups: HashMap<FormId, Group>,
}

impl GlobalRecordStore {
    fn from_esm(&mut self, values: Vec<RawEsmEntry>) {
        for value in values {
            match value {
                RawEsmEntry::Record(record) => {
                    let parsed = match record.parsed() {
                        Ok(value) => value,
                        Err(err) => {
                            warn!(
                                "Failed to parse record ({}: {}): {}",
                                record.form_id, record.ty, err
                            );
                            continue;
                        }
                    };

                    self.mapping.insert(FormId(record.form_id), parsed);
                }
                RawEsmEntry::Group(group) => {
                    let group = match group.parsed() {
                        Ok(value) => value,
                        Err(err) => {
                            warn!(
                                "Failed to parse group ({}: {}): {}",
                                group.label, group.ty, err
                            );
                            continue;
                        }
                    };
                }
            }
        }
    }
}
