use std::path::PathBuf;
use std::fs::File;
use memmap::{Mmap, Protection};
use std::rc::Rc;
use crate::directory::Dir;
//use crate::directory::memory_pointer::MemoryPointer;
use crate::directory::shared_mmap_memory::SharedMmapMemory;
use rand::{thread_rng, Rng};
use regex::Regex;

#[derive(Clone, Debug)]
pub struct SegmentId(pub String);
//随机一个segmentId
pub fn generate_segment_name() -> SegmentId {
    static CHARS: &'static [u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let random_name: String = (0..8)
        .map(|_| thread_rng().choose(CHARS).unwrap().clone() as char)
        .collect();
    SegmentId( String::from("_") + &random_name)
}
#[test]
fn test_new_segment() {
    let SegmentId(segment_name) = generate_segment_name();
    let segment_ptn = Regex::new(r"^_[a-z0-9]{8}$").unwrap();
    assert!(segment_ptn.is_match(&segment_name));
}

pub enum SegmentComponent {
    POSTINGS,
    POSITIONS,
}

pub struct Segment {
    pub directory: Rc<Dir>,
    pub segment_id: SegmentId,
}

impl Segment {
    pub fn path_suffix(component: SegmentComponent) -> &'static str {
        match component {
            SegmentComponent::POSTINGS => ".pstgs",
            SegmentComponent::POSITIONS => ".pos",
        }
    }
    fn get_data(&self, component: SegmentComponent) -> Result<SharedMmapMemory, std::io::Error> {
        self.directory.get_data(&self.segment_id, component)
    }
}
