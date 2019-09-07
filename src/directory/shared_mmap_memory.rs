use std::sync::Arc;
use crate::directory::file::MmapMemory;

#[derive(Clone)]
pub struct SharedMmapMemory(Arc<MmapMemory>);

impl SharedMmapMemory {
    pub fn new(mmap_memory: MmapMemory) -> SharedMmapMemory {
        SharedMmapMemory(Arc::new(mmap_memory))
    }
}