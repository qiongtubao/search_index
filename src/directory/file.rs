use std::path::{PathBuf, Path};
use crate::directory::Dir;
use crate::directory::segment::{SegmentId, Segment, SegmentComponent};
use std::io::Error;
use std::fs::File;
use memmap::{Mmap, Protection};
//use crate::directory::memory_pointer::MemoryPointer;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::directory::shared_mmap_memory::SharedMmapMemory;
use itertools::size_hint::SizeHint;

pub struct MmapMemory(Mmap);

pub struct FileDirectory {
    index_path: PathBuf,
    mmap_cache: RefCell<HashMap<PathBuf, SharedMmapMemory>>
}
impl FileDirectory {
    pub fn for_path(path: PathBuf) -> Self {
        FileDirectory {
            index_path: path,
            mmap_cache: RefCell::new(HashMap::new())
        }
    }
    fn get_or_open_mmap(&self, filepath: &PathBuf) ->Result<SharedMmapMemory, std::io::Error> {
        if !self.mmap_cache.borrow().contains_key(filepath) {
            let file = File::open(filepath)?;
            let memmap = MmapMemory(Mmap::open(&file, Protection::Read)?);
            self.mmap_cache.borrow_mut().insert(filepath.clone(), SharedMmapMemory::new(memmap));
        }
        let shard_map: SharedMmapMemory = self.mmap_cache.borrow().get(filepath).unwrap().clone();
        Ok(shard_map)
    }
}

impl Dir for FileDirectory {
    fn get_data(&self, segment_id: &SegmentId, component: SegmentComponent) -> Result<SharedMmapMemory, Error> {
        let mut filepath = self.index_path.clone();
        let SegmentId(ref segment_id_str) = *segment_id;
        let filename = String::new() + segment_id_str + "." + Segment::path_suffix(component);
        filepath.push(filename);
        self.get_or_open_mmap(&filepath)
    }
}