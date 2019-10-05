use census::TrackedObject;
use crate::core::index::meta::InnerSegmentMeta;
use serde::Serializer;

#[derive(Clone)]
pub struct SegmentMeta {
    pub tracked: TrackedObject<InnerSegmentMeta>,
}

impl serde::Serialize for SegmentMeta {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        self.tracked.serialize(serializer)
    }
}
