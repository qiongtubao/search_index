use std::path::PathBuf;
use std::fs::File;
use memmap::{Mmap, Protection};
use std::rc::Rc;
use crate::directory::Dir;

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
    fn get_file(&self, component: SegmentComponent) -> Result<File, std::io::Error> {
        self.directory.get_file(&self.segment_id, component)
    }
    pub fn open(&self, component: SegmentComponent) ->  Result<Mmap, std::io::Error> {
        let file = self.get_file(component)?;
        Mmap::open(&file, Protection::Read)
    }
}