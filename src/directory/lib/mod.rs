use std::path::Path;
use std::{io, fmt};
use crate::directory::error::OpenReadError;

pub trait DirectoryClone {
    /// Clones the directory and boxes the clone
    fn box_clone(&self) -> Box<dyn Directory>;
}

impl<T> DirectoryClone for T
    where
        T: 'static + Directory + Clone,
{
    fn box_clone(&self) -> Box<dyn Directory> {
        Box::new(self.clone())
    }
}

/**
    'static 是在 Managed::wrap  Box::new() 导致要用'static
*/
pub trait Directory: DirectoryClone + fmt::Debug + 'static {
    //读取小文件用
    fn atomic_read(&self, path: &Path) -> Result<Vec<u8>, OpenReadError>;
    fn atomic_write(&mut self, path: &Path, data: &[u8]) -> io::Result<()>;

    fn exists(&self, path: &Path) -> bool;
}

