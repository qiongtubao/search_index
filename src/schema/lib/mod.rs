pub mod field;
use std::sync::Arc;
pub mod facet;
pub mod value;
pub mod inner;
use inner::InnerSchema;
use crate::schema::lib::build::SchemaBuilder;
use serde::{Serializer, Serialize};
use serde::ser::SerializeSeq;
pub mod build;
#[derive(Clone)]
pub struct Schema(Arc<InnerSchema>);
impl Schema {
    pub fn builder() -> SchemaBuilder {
        SchemaBuilder::default()
    }
}
//格式化Schema对象
impl Serialize for Schema {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        //SerializeSeq => Vec
        let mut seq = serializer.serialize_seq(Some(self.0.fields.len()))?;
        for e in &self.0.fields {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}