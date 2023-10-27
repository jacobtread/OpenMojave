use std::collections::HashMap;
use std::future::ready;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use bsa::v104::ReaderV104;
use bsa::Reader;
use fyrox::asset::io::ResourceIo;
use fyrox::core::futures::future::BoxFuture;
use fyrox::core::io::FileLoadError;
use parking_lot::Mutex;

pub struct BsaResourceIo {
    // Fallback loader for loading files from disk
    fallback: Arc<dyn ResourceIo>,
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

impl BsaResourceIo {
    pub fn load(archives: &[&str], fallback: Arc<dyn ResourceIo>) -> Self {
        let mut paths = HashMap::new();
        let mut archive_handles = HashMap::new();

        // Load all the archives in order
        for archive in archives {
            let archive = archive.trim();
            let file_path = format!("Data/{}", archive);
            let file = match std::fs::File::open(file_path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Failed to load archive \"{}\": {}", archive, err);
                    panic!();
                }
            };

            let mut reader: ArchiveReader = match ReaderV104::read_bsa(BufReader::new(file)) {
                Ok(value) => value,
                Err(err) => {
                    eprintln!("Failed to read archive \"{}\": {}", archive, err);
                    panic!();
                }
            };

            // Read the available directories
            let dirs: Vec<bsa::read::Dir> = match reader.list() {
                Ok(value) => value,
                Err(err) => {
                    eprintln!("Failed to read archive list \"{}\": {}", archive, err);
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

impl ResourceIo for BsaResourceIo {
    fn load_file<'a>(
        &'a self,
        path: &'a std::path::Path,
    ) -> BoxFuture<'a, Result<Vec<u8>, FileLoadError>> {
        let asset_path = match self.paths.get(path) {
            Some(value) => value,
            // Attempt to use the fallback handler for unknown assets
            None => {
                return self.fallback.load_file(path);
            }
        };

        Box::pin(async move {
            let handle = self
                .archive_handles
                .get(&asset_path.archive)
                .expect("Archive handle was missing");

            let handle = &mut *handle.lock();

            let mut out = Vec::new();

            handle.extract(&asset_path.file, &mut out)?;

            Ok(out)
        })
    }

    fn exists<'a>(&'a self, path: &'a std::path::Path) -> BoxFuture<'a, bool> {
        Box::pin(async move { self.paths.contains_key(path) || self.fallback.exists(path).await })
    }

    /// Used to check whether a path is a file
    fn is_file<'a>(&'a self, path: &'a std::path::Path) -> BoxFuture<'a, bool> {
        self.exists(path)
    }

    fn is_dir<'a>(&'a self, path: &'a std::path::Path) -> BoxFuture<'a, bool> {
        // Don't support directory checking
        Box::pin(ready(false))
    }
}
