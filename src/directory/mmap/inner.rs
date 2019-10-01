use std::path::PathBuf;
use std::sync::RwLock;
use crate::directory::mmap::cache::MmapCache;
use crate::directory::error::OpenDirectoryError;
use tempfile::TempDir;
use crate::directory::mmap::watcher_wrapper::WatcherWrapper;

pub struct MmapDirectoryInner {
    pub root_path: PathBuf,
    mmap_cache: RwLock<MmapCache>,
    _temp_directory: Option<TempDir>,
    watcher: RwLock<Option<WatcherWrapper>>,
}

impl MmapDirectoryInner {
    pub fn new(
        root_path: PathBuf,
        temp_directory: Option<TempDir>,
    ) -> Result<MmapDirectoryInner, OpenDirectoryError> {
        //创建MmapDirectoryInner
        let mmap_directory_inner = MmapDirectoryInner {
            root_path,
            mmap_cache: Default::default(),
            _temp_directory: temp_directory,
            watcher: RwLock::new(None),
        };
        Ok(mmap_directory_inner)
    }
}