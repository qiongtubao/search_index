use std::path::PathBuf;
use crate::directory::error::IOError;
//use crate::directory::error::LockError;
#[derive(Debug)]
pub struct DataCorruption {
    filepath: Option<PathBuf>,
    comment: String,
}
#[derive(Debug)]
pub enum TantivyError {
    PathDoesNotExist(PathBuf),
    FileAlreadyExists(PathBuf),
    IndexAlreadyExists,
//    LockFailure(LockError, Option<String>),
    IOError(IOError),
    DataCorruption(DataCorruption),
    Poisoned,
    InvalidArgument(String),
    ErrorInThread(String),
    SchemaError(String),
    SystemError(String),
}