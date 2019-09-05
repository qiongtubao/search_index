mod segment;
use std::path::PathBuf;
use crate::directory::segment::{SegmentId, Segment, SegmentComponent};
use std::rc::Rc;
use std::io::Error;
use crate::directory::file::FileDirectory;
use std::fs::File;

mod file;
pub trait Dir {
    fn get_file<'a>(&'a self, segment_id: &SegmentId, component: SegmentComponent) -> Result<File, Error>; // {
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
        Directory {
            dir: Rc::new(FileDirectory::for_path(path))
        }
    }
}
impl Dir for Directory {
    fn get_file(&self, segment_id: &SegmentId, component: SegmentComponent) -> Result<File, Error> {
        self.dir.get_file(segment_id, component)
    }
}
