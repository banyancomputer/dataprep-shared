use jwalk::DirEntry;
use serde::{Deserialize, Serialize};
use std::fs::Metadata;
use std::path::PathBuf;

#[derive(Debug)]
pub struct SpiderMetadata {
    /// this is the root of the backup
    pub original_root: PathBuf,
    /// this is the path relative to the root of the backup
    pub original_location: PathBuf,
    /// this is the canonicalized path of the original file
    pub canonicalized_path: PathBuf,
    /// this is the metadata of the original file
    pub original_metadata: Metadata,
}

pub fn make_spider_metadata(entry: DirEntry<((), ())>, input_root: PathBuf) -> SpiderMetadata {
    let original_root = input_root;
    let original_location = entry
        .path()
        .strip_prefix(&original_root)
        .unwrap()
        .to_path_buf();
    let canonicalized_path = entry.path().canonicalize().unwrap();
    let original_metadata = entry.metadata().unwrap();
    SpiderMetadata {
        original_root,
        original_location,
        canonicalized_path,
        original_metadata,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileType {
    Directory,
    Symlink,
    File,
}

/// for getting the metadata you want in the manifest from the Metadata object onto disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataToDisk {
    file_type: FileType,
    len: u64,
    permissions: (), //TODO: figure out how to get permissions
    modified: (),    //TODO: figure out how to get modified
    accessed: (),    //TODO: figure out how to get accessed
    created: (),     //TODO: figure out how to get created
    owner: (),       //TODO: figure out how to get owner
                     // TODO come up with more metadata to store
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// for getting the metadata you want in the manifest from the Metadata object onto disk.
pub struct SpiderMetadataToDisk {
    pub original_root: PathBuf,
    /// this is the path relative to the root of the backup
    pub original_location: PathBuf,
    pub canonicalized_path: PathBuf,
    pub original_metadata: MetadataToDisk,
}

impl From<&SpiderMetadata> for SpiderMetadataToDisk {
    fn from(value: &SpiderMetadata) -> Self {
        let original_root = value.original_root.clone();
        let original_location = value.original_location.clone();
        let canonicalized_path = value.canonicalized_path.clone();
        let original_metadata = MetadataToDisk {
            file_type: match value.original_metadata.file_type().is_dir() {
                true => FileType::Directory,
                false => match value.original_metadata.file_type().is_symlink() {
                    true => FileType::Symlink,
                    false => FileType::File,
                },
            },
            len: value.original_metadata.len(),
            permissions: (),
            modified: (),
            accessed: (),
            created: (),
            owner: (),
        };
        SpiderMetadataToDisk {
            original_root,
            original_location,
            canonicalized_path,
            original_metadata,
        }
    }
}
