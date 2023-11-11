use bevy::asset::io::{
    AssetReaderError, AssetSource, AssetSourceBuilder, AssetSourceBuilders, AssetSourceId,
    PathStream, Reader as BevyReader, VecReader,
};
use bevy::prelude::*;
use bevy::utils::BoxedFuture;
use bevy::{asset::io::AssetReader, utils::hashbrown::HashMap};
use bsa::Reader;
use bsa::{read::Dir as BsaDir, read::File as BsaFile, ReaderV104 as BsaReader};
use parking_lot::Mutex;
use std::fs::File as StdFile;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use crate::utils::config::GameConfiguration;

/// Asset reader for loading assets from within .bsa archive files
pub struct BsaAssetReader {
    fallback: Box<dyn AssetReader>,
    /// Mapping between paths, the file, and which archive its apart of
    files: HashMap<PathBuf, (BsaFile, ArchiveIndex)>,
    /// Handles to the opened archives
    archives: Vec<ArchiveHandle>,
}

impl BsaAssetReader {
    pub fn new(fallback: Box<dyn AssetReader>) -> Self {
        Self {
            fallback,
            files: Default::default(),
            archives: Default::default(),
        }
    }
}

/// Plugin for swapping the default asset server [`AssetReader`]
/// with the [`BsaAssetReader`] using the original one as its fallback
pub struct BsaPlugin;

impl Plugin for BsaPlugin {
    fn build(&self, app: &mut App) {
        let config = app.world.resource::<GameConfiguration>();

        // Collect the archives that must be loaded
        let archives: Vec<String> = config
            .archive
            .SArchiveList
            // Split the comma seperated archive list
            .split(", ")
            // Trim and create owned values from the archive names
            .map(|value| value.trim().to_string())
            .collect();

        let mut sources = app
            .world
            .get_resource_or_insert_with::<AssetSourceBuilders>(Default::default);

        sources.insert(
            AssetSourceId::Default,
            AssetSource::build()
                // Provide a reader
                .with_reader(move || {
                    let fallback = AssetSource::get_default_reader("Data".to_string())();
                    let mut reader = BsaAssetReader::new(fallback);

                    for archive in &archives {
                        debug!("Loading archive \"{}\"", archive);

                        if let Err(err) = reader.add_archive(archive) {
                            error!("Failed to load archive \"{}\": {}", archive, err);
                        }
                    }

                    Box::new(reader)
                }),
            // Use default source for writers:
            // .with_writer(AssetSource::get_default_writer("Data".to_string())),
        );
    }
}

/// Handle to access a .bsa archive
pub struct ArchiveHandle {
    /// Shared access to the archive reader behind a mutex
    inner: Mutex<BsaReader<BufReader<StdFile>>>,
}

/// Index of a loaded archive
#[derive(Debug, Clone, Copy)]
pub struct ArchiveIndex(usize);

impl BsaAssetReader {
    pub fn add_archive(&mut self, archive_name: &str) -> std::io::Result<()> {
        let full_path = format!("Data/{}", archive_name.trim());

        // Open a reader to the archive file
        let reader = BufReader::new(StdFile::open(full_path)?);
        let mut reader: BsaReader<_> = BsaReader::read_bsa(reader)?;

        // Read the directories present in the archive
        let dirs: Vec<BsaDir> = reader.list()?;

        // Create the archive index (Will be the next index after insertion)
        let archive_index: ArchiveIndex = ArchiveIndex(self.archives.len());

        // Extend the files map with the files present in the archive
        self.files.extend(
            dirs.into_iter()
                // Only visit named directories and create dir path
                .filter_map(|mut dir| {
                    let dir_name: String = dir.id.name.take()?;
                    let dir_path: PathBuf = PathBuf::from(dir_name);

                    Some((dir_path, dir.files))
                })
                // Flatten the iteration over the files
                .flat_map(|(dir_path, files)| {
                    files
                        .into_iter()
                        // Take only named files and map to insertion format
                        .filter_map(move |file| {
                            let file_name: &str = file.id.name.as_ref()?;
                            let file_path: PathBuf = dir_path.join(file_name);

                            Some((file_path, (file, archive_index)))
                        })
                }),
        );

        // Add the archive handle
        self.archives.push(ArchiveHandle {
            inner: Mutex::new(reader),
        });

        debug!("Loaded archive \"{}\"", archive_name);

        Ok(())
    }
}

impl AssetReader for BsaAssetReader {
    fn read<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<BevyReader<'a>>, AssetReaderError>> {
        let (file, archive_index) = match self.files.get(path) {
            Some(value) => value,
            // File missing attempt to read from fallback
            None => return self.fallback.read(path),
        };

        Box::pin(async move {
            // Archive **must** exists since theres no way to remove archives
            let handle = &self.archives[archive_index.0];

            let reader = &mut *handle.inner.lock();

            // Extract the file to a buffer internally
            let mut buffer = Vec::new();
            reader.extract(file, &mut buffer)?;

            // Provide it the buffer through a VecReader
            let reader: Box<BevyReader<'a>> = Box::new(VecReader::new(buffer));
            Ok(reader)
        })
    }

    fn read_meta<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<BevyReader<'a>>, AssetReaderError>> {
        self.fallback.read_meta(path)
    }

    fn read_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<PathStream>, AssetReaderError>> {
        self.fallback.read_directory(path)
    }

    fn is_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<bool, AssetReaderError>> {
        self.fallback.is_directory(path)
    }
}
