use std::path::Path;
use std::io;
use crate::directory::error::OpenReadError;

pub trait Directory {
    //读取小文件用
    fn atomic_read(&self, path: &Path) -> Result<Vec<u8>, OpenReadError>;
    fn atomic_write(&mut self, path: &Path, data: &[u8]) -> io::Result<()>;

    fn exists(&self, path: &Path) -> bool;
}