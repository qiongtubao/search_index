use crate::core::segment::id::SegmentId;
use census::{Inventory};
use crate::core::segment::meta::SegmentMeta;
use crate::schema::lib::Schema;

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
#[derive(Clone, Serialize)]
pub struct IndexMeta {
    /// List of `SegmentMeta` informations associated to each finalized segment of the index.
    pub segments: Vec<SegmentMeta>,
    /// Index `Schema`
    pub schema: Schema,
    /// Opstamp associated to the last `commit` operation.
    pub opstamp: Opstamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Payload associated to the last commit.
    ///
    /// Upon commit, clients can optionally add a small `Striing` payload to their commit
    /// to help identify this commit.
    /// This payload is entirely unused by tantivy.
    pub payload: Option<String>,
}

impl IndexMeta {
    pub fn with_schema(schema: Schema) -> IndexMeta {
        IndexMeta {
            segments: vec![],
            schema,
            opstamp: 0u64,
            payload: None,
        }
    }
}