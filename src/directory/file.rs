use std::path::PathBuf;
use crate::directory::Dir;
use crate::directory::segment::{SegmentId, Segment, SegmentComponent};
use std::io::Error;
use std::fs::File;
use memmap::{Mmap, Protection};
use crate::directory::memory_pointer::MemoryPointer;
struct MmapMemory(Mmap);

pub struct FileDirectory {
    index_path: PathBuf,
}
impl FileDirectory {
    pub fn for_path(path: PathBuf) -> Self {
        FileDirectory {
            index_path: path
        }
    }
}

impl Dir for FileDirectory {
    fn get_file<'a>(&'a self, segment_id: &SegmentId, component: SegmentComponent) -> Result<&'a MemoryPointer, Error> {
        let mut res = self.index_path.clone();
        let SegmentId(ref segment_id_str) = *segment_id;
        let filename = String::new() + segment_id_str + "." + Segment::path_suffix(component);
        res.push(filename);
        let file = File::open(res)?;
        let mmap = MmapMemory(Mmap::open(&file, Protection::Read)?);
        Err(std::io::Error::new(std::io::ErrorKind::AddrInUse, "mmap"))
    }
}