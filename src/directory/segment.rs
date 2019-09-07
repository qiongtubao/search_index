use std::path::PathBuf;
use std::fs::File;
use memmap::{Mmap, Protection};
use std::rc::Rc;
use crate::directory::Dir;
//use crate::directory::memory_pointer::MemoryPointer;
use crate::directory::shared_mmap_memory::SharedMmapMemory;

#[derive(Clone, Debug)]
pub struct SegmentId(pub String);

pub enum SegmentComponent {
    POSTINGS,
    POSITIONS,
}

pub struct Segment {
    pub directory: Rc<Dir>,
    pub segment_id: SegmentId,
}

impl Segment {
    pub fn path_suffix(component: SegmentComponent) -> &'static str{
        match component {
            SegmentComponent::POSTINGS => ".pstgs",
            SegmentComponent::POSITIONS => ".pos",
        }
    }
    fn get_data(&self, component: SegmentComponent) -> Result<SharedMmapMemory, std::io::Error> {
        self.directory.get_data(&self.segment_id, component)
    }
}

