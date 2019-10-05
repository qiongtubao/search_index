use std::path::PathBuf;
use crate::directory::error::IOError;
use std::io;

//use crate::directory::error::LockError;
#[derive(Debug)]
pub struct DataCorruption {
    filepath: Option<PathBuf>,
    comment: String,
}
impl DataCorruption {
    pub fn new(filepath: PathBuf, comment: String) -> DataCorruption {
        DataCorruption {
            filepath: Some(filepath),
            comment,
        }
    }
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

impl From<IOError> for TantivyError {
    fn from(io_error: IOError) -> TantivyError {
        TantivyError::IOError(io_error)
    }
}

impl From<DataCorruption> for TantivyError {
    fn from(data_corruption: DataCorruption) -> TantivyError {
        TantivyError::DataCorruption(data_corruption)
    }
}
impl From<io::Error> for TantivyError {
    fn from(io_error: io::Error) -> TantivyError {
        TantivyError::IOError(io_error.into())
    }
}
impl From<serde_json::Error> for TantivyError {
    fn from(error: serde_json::Error) -> TantivyError {
        let io_err = io::Error::from(error);
        TantivyError::IOError(io_err.into())
    }
}