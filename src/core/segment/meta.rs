use census::TrackedObject;
use crate::core::index::meta::InnerSegmentMeta;

#[derive(Clone)]
pub struct SegmentMeta {
    tracked: TrackedObject<InnerSegmentMeta>,
}