use std::fs::{create_dir_all, File};
use std::io::{BufReader, Seek};
use std::path::Path;

use bevy::prelude::*;
use bevy::utils::HashSet;
use binrw::BinRead;
use bsa::v104::ReaderV104;
use bsa::{read, Reader};
use esm::RecordHeader;
use rayon::prelude::{ParallelBridge, ParallelIterator};
mod esm;

fn main() {
    std::fs::read_dir("Data")
        .unwrap()
        .par_bridge()
        .filter_map(|value| value.ok())
        .filter(|value| value.file_name().to_string_lossy().ends_with(".bsa"))
        .for_each(|value| {
            let mut file = std::fs::File::open(value.path()).unwrap();
            let mut reader = ReaderV104::read_bsa(&mut file).unwrap();
            let entries: Vec<read::Dir> = reader.list().unwrap();
            for entry in entries {
                for file in entry.files {
                    let mut out = Path::new("DataUnpack").join(value.file_name()).join(
                        file.id
                            .name
                            .clone()
                            .unwrap_or_else(|| "Unknown".to_string()),
                    );
                    if let Some(parent) = out.parent() {
                        create_dir_all(parent).unwrap();
                    }

                    let mut out = File::create(out).unwrap();
                    reader.extract(&file, &mut out).unwrap();
                }
            }
        })

    // App::new().add_plugins(DefaultPlugins).run();
}
