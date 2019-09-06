use std::path::PathBuf;
use std::fs::File;
use memmap::{Mmap, Protection};
use std::rc::Rc;
use crate::directory::Dir;
use crate::directory::memory_pointer::MemoryPointer;

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
    fn get_file<'a>(&'a self, component: SegmentComponent) -> Result<&'a MemoryPointer, std::io::Error> {
        self.directory.get_file(&self.segment_id, component)
    }
}

