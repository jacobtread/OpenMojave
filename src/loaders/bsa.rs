//! Asset loader for .bsa packed asset bundles

use crate::config::GameConfiguration;
use bevy::utils::HashMap;
use bevy::{asset::AssetIo, prelude::*};
use bsa::v104::ReaderV104;
use bsa::Reader;
use rayon::prelude::{ParallelBridge, ParallelIterator};
use std::fs::create_dir_all;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::fnt::{BitmapFont, FntFontLoader};

pub struct BsaAssetPlugin;

impl Plugin for BsaAssetPlugin {
    fn build(&self, app: &mut App) {
        let config = app
            .world
            .get_resource::<GameConfiguration>()
            .expect("Game configuration was missing from bsa startup");

        let fallback = AssetPlugin {
            asset_folder: "Data".to_string(),
            ..Default::default()
        }
        .create_platform_default_asset_io();

        // Collect the archives that must be loaded
        let archives: Vec<&str> = config.archive.SArchiveList.split(", ").collect();

        let asset_io = BsaAssetIo::load(&archives, fallback);

        let server = AssetServer::new(asset_io);

        app.insert_resource(server);
        app.init_asset_loader::<FntFontLoader>();
        app.add_asset::<BitmapFont>();
    }
}

pub struct BsaAssetIo {
    // Fallback loader for loading files from disk
    fallback: Box<dyn AssetIo>,
    /// Asset path mappings
    paths: HashMap<PathBuf, BsaAssetPath>,
    /// Handles to the individual archive files
    archive_handles: HashMap<String, ArchiveHandle>,
}

type ArchiveReader = ReaderV104<BufReader<std::fs::File>>;
type ArchiveHandle = Arc<Mutex<ArchiveReader>>;

pub struct BsaAssetPath {
    /// File information within the archive
    file: bsa::read::File,
    /// The archive the file is in
    archive: String,
}

impl BsaAssetIo {
    /// Creates a new BSA asset io from the
    pub fn load(archives: &[&str], fallback: Box<dyn AssetIo>) -> Self {
        let mut paths = HashMap::new();
        let mut archive_handles = HashMap::new();

        // Load all the archives in order
        for archive in archives {
            let archive = archive.trim();
            let file_path = format!("Data/{}", archive);
            let file = match std::fs::File::open(file_path) {
                Ok(file) => file,
                Err(err) => {
                    error!("Failed to load archive \"{}\": {}", archive, err);
                    panic!();
                }
            };

            let mut reader: ArchiveReader = match ReaderV104::read_bsa(BufReader::new(file)) {
                Ok(value) => value,
                Err(err) => {
                    error!("Failed to read archive \"{}\": {}", archive, err);
                    panic!();
                }
            };

            // Read the available directories
            let dirs: Vec<bsa::read::Dir> = match reader.list() {
                Ok(value) => value,
                Err(err) => {
                    error!("Failed to read archive list \"{}\": {}", archive, err);
                    panic!();
                }
            };

            for dir in dirs {
                let dir_name = dir.id.name.as_ref().expect("Directory missing name");
                for file in dir.files {
                    let file_name = file.id.name.as_ref().expect("File missing name");

                    let mut path = PathBuf::from_str(&dir_name)
                        .expect("Invalid directory path")
                        .join(file_name);

                    // Create the file path mapping
                    paths.insert(
                        path,
                        BsaAssetPath {
                            file,
                            archive: archive.to_string(),
                        },
                    );
                }
            }

            // Add the handle mapping
            archive_handles.insert(archive.to_string(), Arc::new(Mutex::new(reader)));
        }

        // NOTE: This is code to unpack all the assets
        //
        // let mut data_unpacked = Path::new("DataUnpacked");
        // paths.iter().par_bridge().for_each(|(path, asset_path)| {
        //     let out_path = data_unpacked.join(path);
        //     if out_path.exists() {
        //         return;
        //     }
        //     let handle = archive_handles
        //         .get(&asset_path.archive)
        //         .expect("Archive handle was missing");
        //     let handle = &mut *handle.blocking_lock();
        //     if let Some(parent) = out_path.parent() {
        //         create_dir_all(parent).unwrap();
        //     }
        //     let mut out = std::fs::File::create(out_path).unwrap();
        //     if let Err(err) = handle.extract(&asset_path.file, &mut out) {
        //         error!("Failed to extract file {}: {}", path.display(), err);
        //     }
        // });

        Self {
            archive_handles,
            paths,
            fallback,
        }
    }
}

impl AssetIo for BsaAssetIo {
    fn load_path<'a>(
        &'a self,
        path: &'a std::path::Path,
    ) -> bevy::utils::BoxedFuture<'a, Result<Vec<u8>, bevy::asset::AssetIoError>> {
        let asset_path = match self.paths.get(path) {
            Some(value) => value,
            // Attempt to use the fallback handler for unknown assets
            None => {
                return self.fallback.load_path(path);
            }
        };

        Box::pin(async move {
            let handle = self
                .archive_handles
                .get(&asset_path.archive)
                .expect("Archive handle was missing");

            let handle = &mut *handle.lock().await;

            let mut out = Vec::new();

            handle.extract(&asset_path.file, &mut out)?;

            Ok(out)
        })
    }

    fn read_directory(
        &self,
        path: &std::path::Path,
    ) -> Result<Box<dyn Iterator<Item = std::path::PathBuf>>, bevy::asset::AssetIoError> {
        self.fallback.read_directory(path)
    }

    fn watch_path_for_changes(
        &self,
        to_watch: &std::path::Path,
        to_reload: Option<std::path::PathBuf>,
    ) -> Result<(), bevy::asset::AssetIoError> {
        self.fallback.watch_path_for_changes(to_watch, to_reload)
    }

    fn watch_for_changes(
        &self,
        configuration: &bevy::asset::ChangeWatcher,
    ) -> Result<(), bevy::asset::AssetIoError> {
        self.fallback.watch_for_changes(configuration)
    }

    fn get_metadata(
        &self,
        path: &std::path::Path,
    ) -> Result<bevy::asset::Metadata, bevy::asset::AssetIoError> {
        if self.paths.contains_key(path) {
            Ok(bevy::asset::Metadata::new(bevy::asset::FileType::File))
        } else {
            self.fallback.get_metadata(path)
        }
    }
}
