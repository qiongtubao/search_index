use crate::directory::lib::Directory;
use std::sync::{RwLock, Arc, RwLockWriteGuard};
use std::collections::HashSet;
use std::path::{PathBuf, Path};
use crate::Result;
use crate::directory::MANAGED_FILEPATH;
use crate::directory::error::{OpenReadError, LockError};
use crate::error::DataCorruption;
use std::io::Write;
use crate::directory::lib::lock::{Lock, DirectoryLock};

#[derive(Debug, Default)]
struct MetaInformation {
    managed_paths: HashSet<PathBuf>,
}

#[derive(Debug)]
pub struct ManagedDirectory {
    //为什么用box?
    directory: Box<dyn Directory>,
    meta_informations: Arc<RwLock<MetaInformation>>,
}

fn is_managed(path: &Path) -> bool {
    path.to_str().map(|p_str| !p_str.starts_with(".")).unwrap_or(true)
}

fn save_managed_paths(directory: &mut dyn Directory, wlock: &RwLockWriteGuard<'_, MetaInformation>) -> std::io::Result<()> {
    let mut w = serde_json::to_vec(&wlock.managed_paths)?;
    writeln!(&mut w)?;
    directory.atomic_write(&MANAGED_FILEPATH, &w[..])?;
    Ok(())
}

impl ManagedDirectory {
    pub fn wrap<Dir: Directory>(directory: Dir) -> Result<ManagedDirectory> {
        //读取文件夹下的.managed.json
        match directory.atomic_read(&MANAGED_FILEPATH) {
            Ok(data) => {
                //转换成str
                let managed_files_json = String::from_utf8_lossy(&data);
                //转换成hashset
                let managed_files: HashSet<PathBuf> = serde_json::from_str(&managed_files_json).map_err(|e| {
                    //转换错误代码
                    DataCorruption::new(
                        MANAGED_FILEPATH.to_path_buf(),
                        format!("Managed file cannot be deserialized: {:?}. ", e),
                    )
                })?;
                Ok(ManagedDirectory {
                    directory: Box::new(directory),
                    meta_informations: Arc::new(RwLock::new(MetaInformation {
                        managed_paths: managed_files
                    })),
                })
            },
            Err(OpenReadError::FileDoesNotExist(_)) => Ok(ManagedDirectory {
                directory: Box::new(directory),
                meta_informations: Arc::default(),
            }),
            Err(OpenReadError::IOError(e)) => Err(From::from(e)),
        }
    }
    fn register_file_as_managed(&mut self, filepath: &Path) -> std::io::Result<()> {
        //暂时还不理解为什么用.来判断
        if !is_managed(filepath) {
            return Ok(())
        }
        let mut meta_wlock = self.meta_informations
            .write().expect("Managed file lock poisoned");
        //数据内添加filepath
        let has_changed = meta_wlock.managed_paths.insert(filepath.to_owned());
        if has_changed {
            //写入文件
            save_managed_paths(self.directory.as_mut(), &meta_wlock)?;
        }
        Ok(())
    }

}

//Index clone

impl Clone for ManagedDirectory {
    fn clone(&self) -> ManagedDirectory {
        ManagedDirectory {
            directory: self.directory.box_clone(),
            meta_informations: Arc::clone(&self.meta_informations),
        }
    }
}

impl Directory for ManagedDirectory {
    fn atomic_read(&self, path: &Path) -> std::result::Result<Vec<u8>, OpenReadError> {
        self.directory.atomic_read(path)
    }

    fn atomic_write(&mut self, path: &Path, data: &[u8]) -> std::io::Result<()> {
        self.register_file_as_managed(path)?;
        self.directory.atomic_write(path, data)
    }

    fn exists(&self, path: &Path) -> bool {
        self.directory.exists(path)
    }

    fn acquire_lock(&self, lock: &Lock) -> std::result::Result<DirectoryLock, LockError> {
        self.directory.acquire_lock(lock)
    }
}