use crate::directory::lib::Directory;
use std::sync::{RwLock, Arc};
use std::collections::HashSet;
use std::path::{PathBuf, Path};
use crate::Result;
use crate::directory::MANAGED_FILEPATH;
use crate::directory::error::OpenReadError;
use crate::error::DataCorruption;

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
        unimplemented!()
    }

    fn atomic_write(&mut self, path: &Path, data: &[u8]) -> std::io::Result<()> {
        unimplemented!()
    }

    fn exists(&self, path: &Path) -> bool {
        unimplemented!()
    }
}