use crate::common::check_done_reading;
use crate::error::Error;
use binrw::{binrw, BinRead, BinWrite};
use encoding_rs::WINDOWS_1252;
use serde_derive::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{Cursor, Read, Seek};
use std::path::PathBuf;
use std::str;

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryEntry {
    pub string_id: u32,
    pub offset: u32,
}

#[binrw]
#[br(import(length_prefixed: bool))]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawStringTable {
    #[br(calc(length_prefixed))]
    #[bw(ignore)]
    pub length_prefixed: bool,

    pub num_entries: u32,
    pub string_data_size: u32,

    #[br(count = num_entries)]
    pub entries: Vec<DirectoryEntry>,

    #[br(count = string_data_size)]
    pub data: Vec<u8>,
}

impl RawStringTable {
    pub fn parse<T: Read + Seek>(reader: &mut T, length_prefixed: bool) -> Result<Self, Error> {
        let result = Self::read_args(reader, (length_prefixed,))?;
        check_done_reading(reader)?;
        Ok(result)
    }
}

fn get_str(raw: &RawStringTable, offset: usize, length_prefixed: bool) -> Result<String, Error> {
    let bytes = if length_prefixed {
        let mut cursor = Cursor::new(&raw.data[offset..offset + 4]);
        let length = u32::read_le(&mut cursor)? as usize;

        &raw.data[offset + 4..offset + 4 + length]
    } else {
        let end = raw.data[offset..]
            .iter()
            .position(|&c| c == b'\0')
            .ok_or(Error::StringEOF)?;

        &raw.data[offset..offset + end]
    };

    let (s, _, _) = WINDOWS_1252.decode(bytes);
    Ok(s.into())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TableType {
    STRINGS,
    DLSTRINGS,
    ILSTRINGS,
}

impl TableType {
    pub fn extension(&self) -> String {
        match self {
            Self::STRINGS => "STRINGS".into(),
            Self::DLSTRINGS => "DLSTRINGS".into(),
            Self::ILSTRINGS => "ILSTRINGS".into(),
        }
    }

    pub fn length_prefixed(&self) -> bool {
        match self {
            Self::STRINGS => false,
            Self::DLSTRINGS => true,
            Self::ILSTRINGS => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StringID(u32);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Offset(u32);

#[derive(Debug)]
pub struct StringTable {
    strings: Vec<String>,
    offsets: HashMap<StringID, usize>,
    length_prefixed: bool,
}

impl StringTable {
    pub fn load(path: &PathBuf, length_prefixed: bool) -> Result<Self, Error> {
        let mut strings = Vec::new();
        let mut offsets = HashMap::new();

        let mut f = File::open(path)?;
        let rst = RawStringTable::parse(&mut f, length_prefixed)?;

        let mut offset_to_index = rst
            .entries
            .iter()
            .map(|e| e.offset)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        offset_to_index.sort();
        for offset in &offset_to_index {
            let asdf = get_str(&rst, *offset as usize, length_prefixed)?;
            strings.push(asdf);
        }

        let offset_indices = offset_to_index
            .iter()
            .enumerate()
            .map(|(k, v)| (v, k))
            .collect::<HashMap<_, _>>();

        for entry in &rst.entries {
            offsets.insert(StringID(entry.string_id), offset_indices[&entry.offset]);
        }

        Ok(Self {
            strings,
            offsets,
            length_prefixed,
        })
    }

    pub fn serialize(&self) -> Result<Vec<u8>, Error> {
        let mut data: Vec<u8> = Vec::new();
        let mut entries = Vec::new();
        let mut offsets = Vec::new();

        for string in &self.strings {
            let encoded = WINDOWS_1252.encode(string).0.to_vec();
            offsets.push(data.len() as u32);
            if self.length_prefixed {
                data.extend_from_slice(&(encoded.len() as u32).to_le_bytes());
            }
            data.extend_from_slice(&encoded);
            if !self.length_prefixed {
                data.push(0);
            }
        }
        let mut index_to_offset = self.offsets.iter().collect::<Vec<_>>();
        index_to_offset.sort();
        for (string_id, offset) in index_to_offset {
            entries.push(DirectoryEntry {
                string_id: string_id.0,
                offset: offsets[*offset],
            });
        }

        let x = RawStringTable {
            length_prefixed: self.length_prefixed,
            num_entries: entries.len() as u32,
            string_data_size: data.len() as u32,
            entries,
            data,
        };

        let mut result = Cursor::new(Vec::new());
        x.write(&mut result)?;
        Ok(result.into_inner())
    }

    pub fn get_string(&self, id: &u32) -> Option<&String> {
        self.strings.get(self.offsets[&StringID(*id)])
    }
}

pub struct StringTables {
    pub tables: HashMap<(String, TableType), StringTable>,
}

impl StringTables {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    pub fn load_plugin_path(&mut self, path: &str, language: &str) -> Result<(), Error> {
        let path = PathBuf::from(path);
        let plugin_name = path.file_stem().unwrap().to_string_lossy();
        let dir = path.parent().unwrap().join("Strings");

        use TableType::*;

        for suffix in [STRINGS, DLSTRINGS, ILSTRINGS] {
            let full_path = dir.join(format!(
                "{}_{}.{}",
                plugin_name,
                language,
                suffix.extension()
            ));

            let table = StringTable::load(&full_path, suffix.length_prefixed())?;

            self.tables.insert((plugin_name.to_string(), suffix), table);
        }

        Ok(())
    }

    pub fn get_string(&self, id: &u32) -> Option<&String> {
        for table in self.tables.values() {
            if let Some(s) = table.get_string(id) {
                return Some(s);
            }
        }
        None
    }
}
