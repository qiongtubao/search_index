use crate::directory::mmap::inner::MmapDirectoryInner;
use std::sync::Arc;
use std::path::{Path, PathBuf};
use crate::directory::error::OpenDirectoryError;
use std::ops::Deref;
use tempfile::TempDir;
mod inner_watcher_wrapper;
mod watcher_wrapper;
mod cache;
mod inner;
pub type BoxedData = Box<dyn Deref<Target = [u8]> + Send + Sync + 'static>;
pub struct MmapDirectory {
    inner: Arc<MmapDirectoryInner>,
}

impl MmapDirectory {
    fn new(root_path: PathBuf, temp_directory: Option<TempDir>) -> Result<MmapDirectory, OpenDirectoryError> {
        let inner = MmapDirectoryInner::new(root_path, temp_directory)?;
        Ok(MmapDirectory {
            inner: Arc::new(inner)
        })
    }
    pub fn open<P: AsRef<Path>>(directory_path: P) -> Result<MmapDirectory, OpenDirectoryError> {
        let directory_path: &Path = directory_path.as_ref();
        if !directory_path.exists() {
            Err(OpenDirectoryError::DoesNotExist(PathBuf::from(
                directory_path,
            )))
        }else if !directory_path.is_dir() {
            Err(OpenDirectoryError::NotADirectory(PathBuf::from(directory_path)))
        }else{
            MmapDirectory::new(PathBuf::from(directory_path), None)
        }
    }
}