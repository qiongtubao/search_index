pub mod field;
use std::sync::Arc;
pub mod facet;
pub mod value;
pub mod inner;
use inner::InnerSchema;
use crate::schema::lib::build::SchemaBuilder;
use serde::{Serializer, Serialize, Deserialize, Deserializer};
use serde::ser::SerializeSeq;
use serde::de::{Visitor, SeqAccess};
use std::fmt;
use std::collections::HashMap;

pub mod build;
#[derive(Clone)]
pub struct Schema(Arc<InnerSchema>);
impl Schema {
    pub fn builder() -> SchemaBuilder {
        SchemaBuilder::default()
    }
}
/**
    Schema对象 -> Array
*/
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
/**
    Array 对象转换成Schema
*/
impl<'de> Deserialize<'de> for Schema {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct SchemaVisitor;

        impl<'de> Visitor<'de> for SchemaVisitor {
            type Value = Schema;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("struct Schema")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: SeqAccess<'de>,
            {
                let mut schema = SchemaBuilder {
                    fields: Vec::with_capacity(seq.size_hint().unwrap_or(0)),
                    fields_map: HashMap::with_capacity(seq.size_hint().unwrap_or(0)),
                };

                while let Some(value) = seq.next_element()? {
                    schema.add_field(value);
                }

                Ok(schema.build())
            }
        }

        deserializer.deserialize_seq(SchemaVisitor)
    }
}