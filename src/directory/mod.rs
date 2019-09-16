mod segment;
use crate::directory::file::FileDirectory;
use crate::directory::segment::{generate_segment_name, Segment, SegmentComponent, SegmentId};
use std::fs::File;
use std::io::Error;
use std::path::PathBuf;
use std::rc::Rc;
//use crate::directory::memory_pointer::MemoryPointer;
use crate::directory::mem::MemDirectory;
use crate::directory::shared_mmap_memory::SharedMmapMemory;

mod file;
mod mem;
mod memory_pointer;
mod shared_mmap_memory;
pub trait Dir {
    fn get_data(
        &self,
        segment_id: &SegmentId,
        component: SegmentComponent,
    ) -> Result<SharedMmapMemory, Error>; // {
}
#[derive(Clone)]
pub struct Directory {
    dir: Rc<Dir>,
}
impl Directory {
    fn segment(&self, segment_id: &SegmentId) -> Segment {
        Segment {
            directory: self.dir.clone(),
            segment_id: segment_id.clone(),
        }
    }
    pub fn open(path_str: &str) -> Directory {
        let path = PathBuf::from(path_str);
        Directory::from(FileDirectory::for_path(path))
    }
    pub fn new_segment(&self) -> Segment {
        self.segment(&generate_segment_name())
    }
    fn from<T: Dir + 'static>(directory: T) -> Directory {
        Directory {
            dir: Rc::new(directory),
        }
    }
    pub fn in_mem() -> Directory {
        Directory::from(MemDirectory::new())
    }
}
impl Dir for Directory {
    fn get_data(
        &self,
        segment_id: &SegmentId,
        component: SegmentComponent,
    ) -> Result<SharedMmapMemory, Error> {
        self.dir.get_data(segment_id, component)
    }
}
