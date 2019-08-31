use std::path::PathBuf;
use std::fs::File;
use memmap::{Mmap, Protection};
#[derive(Clone, Debug)]
pub struct SegmentId(pub String);

enum SegmentComponent {
    POSTINGS,
    POSITIONS,
}

pub struct SegmentDirectory {
    pub index_path: PathBuf,
    pub segment_id: SegmentId,
}

impl SegmentDirectory {
    fn path_suffix(component: SegmentComponent) -> &'static str{
        match component {
            SegmentComponent::POSTINGS => ".pstgs",
            SegmentComponent::POSITIONS => ".pos",
        }
    }
    fn get_file(&self, component: SegmentComponent) -> Result<File, std::io::Error> {
        let mut res = self.index_path.clone();
        let SegmentId(ref segment_id_str) = self.segment_id;
        let filename = String::new()+ segment_id_str + "." + SegmentDirectory::path_suffix(component);
        res.push(filename);
        File::open(res)
    }
    pub fn open(&self, component: SegmentComponent) ->  Result<Mmap, std::io::Error> {
        let file = self.get_file(component)?;
        Mmap::open(&file, Protection::Read)
    }
}