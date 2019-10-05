use std::path::PathBuf;

pub struct Lock {
    pub filepath: PathBuf,
    pub is_blocking: bool,
}

pub struct DirectoryLock(Box<dyn Send + Sync + 'static>);
