use crate::core::index::Index;
use crate::core::segment::meta::SegmentMeta;

pub mod id;
pub mod meta;
pub mod error;
pub mod updater;
pub struct Segment {
    index: Index,
    meta: SegmentMeta,
}