use crate::core::segment::id::SegmentId;
use census::{Inventory};
#[derive(Clone, Default)]
pub struct SegmentMetaInventory {
    inventory: Inventory<InnerSegmentMeta>,
}




#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerSegmentMeta {
    segment_id: SegmentId,
    max_doc: u32,
    deletes: Option<DeleteMeta>,
}

pub type Opstamp = u64;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct DeleteMeta {
    num_deleted_docs: u32,
    opstamp: Opstamp,
}