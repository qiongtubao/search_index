mod segment;
use std::path::PathBuf;
use crate::directory::segment::{SegmentDirectory, SegmentId};


struct IndexDirectory {
    index_path: PathBuf,
}
impl IndexDirectory {
    pub fn for_path(index_path: PathBuf) -> Self {
        IndexDirectory {
            index_path,
        }
    }
    pub fn read_segment(&self, segment_id: &SegmentId) -> SegmentDirectory {
        SegmentDirectory {
            index_path: self.index_path.clone(),
            segment_id: segment_id.clone(),
        }
    }
}