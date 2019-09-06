use std::path::PathBuf;
use std::collections::HashMap;
use memmap::Mmap;
use crate::directory::memory_pointer::MemoryPointer;
use crate::directory::Dir;
use std::io::Error;
use crate::directory::segment::{SegmentComponent, SegmentId, Segment};

struct MmapMemory(Mmap);
pub struct MemDirectory {
    dir: HashMap<PathBuf, Box<MemoryPointer>>
}

impl MemDirectory {
    pub fn new() -> Self {
        MemDirectory {
            dir: HashMap::new()
        }

    }
}
impl Dir for MemDirectory {
    fn get_file<'a>(&'a self, segment_id: &SegmentId, component: SegmentComponent) -> Result<&MemoryPointer, Error> {
        let SegmentId(ref segment_id_str) = *segment_id;
        let mut path = PathBuf::from(segment_id_str);
        path.push(Segment::path_suffix(component));
        match self.dir.get(&path) {
            Some(buf) => {
                Ok(buf.as_ref())
            },
            None => Err(std::io::Error::new(std::io::ErrorKind::NotFound, "File does not exists"))
        }
    }
}