use std::path::PathBuf;
use std::io;
use once_cell::sync::Lazy;
use crate::directory::lib::lock::Lock;

/**
    打开文件夹错误
*/
#[derive(Debug)]
pub enum OpenDirectoryError {
    /// The underlying directory does not exists.
    DoesNotExist(PathBuf),
    /// The path exists but is not a directory.
    NotADirectory(PathBuf),
    /// IoError
    IoError(io::Error),
}

#[derive(Debug)]
pub enum OpenReadError {
    FileDoesNotExist(PathBuf),
    IOError(IOError),
}

#[derive(Debug)]
pub struct IOError {
    path: Option<PathBuf>,
    err: io::Error,
}

impl Into<io::Error> for IOError {
    fn into(self) -> io::Error {
        self.err
    }
}

impl From<io::Error> for IOError {
    fn from(err: io::Error) -> IOError {
        IOError { path: None, err }
    }
}
#[derive(Debug)]
pub enum LockError {
    LockBusy,
    IOError(io::Error),
}

pub static INDEX_WRITER_LOCK: Lazy<Lock> = Lazy::new(|| Lock {
    filepath: PathBuf::from(".tantivy-writer.lock"),
    is_blocking: false,
});