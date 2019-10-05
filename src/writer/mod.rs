use crate::core::index::Index;
use crate::directory::lib::lock::DirectoryLock;

pub struct IndexWriter {

}
impl IndexWriter {
    pub fn new(index: &Index,
               num_threads: usize,
               heap_size_in_bytes_per_thread: usize,
               directory_lock: DirectoryLock,) -> crate::Result<Self> {
        Ok(IndexWriter {

        })
    }
}