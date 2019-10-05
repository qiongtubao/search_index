use crate::core::index::meta::{InnerSegmentMeta, Opstamp, SegmentMetaInventory, IndexMeta};
use crate::schema::lib::Schema;
use crate::core::segment::meta::SegmentMeta;

//用来转换IndexMeta
#[derive(Deserialize)]
pub struct UntrackedIndexMeta {
    pub segments: Vec<InnerSegmentMeta>,
    pub schema: Schema,
    pub opstamp: Opstamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}

impl UntrackedIndexMeta {
    pub fn track(self, inventory: &SegmentMetaInventory) -> IndexMeta {
        IndexMeta {
            segments: self.segments.into_iter().map(|inner_seg_meta| inner_seg_meta.track(inventory)).collect::<Vec<SegmentMeta>>(),
            schema: self.schema,
            opstamp: self.opstamp,
            payload: self.payload
        }
    }
}