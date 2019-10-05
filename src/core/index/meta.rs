use crate::core::segment::id::SegmentId;
use census::{Inventory};
use crate::core::segment::meta::SegmentMeta;
use crate::schema::lib::Schema;
use crate::core::index::untracked_index_meta::UntrackedIndexMeta;

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

impl InnerSegmentMeta {
    pub fn track(self,  inventory: &SegmentMetaInventory)-> SegmentMeta {
        SegmentMeta {
            tracked: inventory.inventory.track(self)
        }
    }
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
    pub(crate) fn deserialize(
        meta_json: &str,
        inventory: &SegmentMetaInventory,
    ) -> serde_json::Result<IndexMeta> {
        let untracked_meta_json: UntrackedIndexMeta = serde_json::from_str(meta_json)?;
        Ok(untracked_meta_json.track(inventory))
    }
}