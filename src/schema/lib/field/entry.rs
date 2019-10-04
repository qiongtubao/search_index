use crate::schema::lib::field::options::int::IntOptions;
use crate::schema::lib::field::r#type::FieldType;
use crate::schema::lib::field::options::text::TextOptions;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Debug)]
pub struct FieldEntry {
    name: String,
    field_type: FieldType,
}

impl FieldEntry {
    pub fn new_u64(field_name: String, field_type: IntOptions) -> FieldEntry {
        FieldEntry {
            name: field_name,
            field_type: FieldType::U64(field_type),
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn new_text(field_name: String, field_type: TextOptions) -> FieldEntry {
        FieldEntry {
            name: field_name,
            field_type: FieldType::Str(field_type)
        }
    }
    //是否建立索引
    pub fn is_indexed(&self) -> bool {
        match self.field_type {
            FieldType::Str(ref options) => options.get_indexing_options().is_some(),
            FieldType::U64(ref options)
            | FieldType::I64(ref options)
            | FieldType::F64(ref options)
            | FieldType::Date(ref options) => options.is_indexed(),
            FieldType::HierarchicalFacet => true,
            FieldType::Bytes => false,
        }
    }
}
/**
    格式化fieldEntry
*/
impl Serialize for FieldEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        //SerializeStruct对象 => Object
        let mut s = serializer.serialize_struct("field_entry", 3)?;
        s.serialize_field("name", &self.name)?;

        match self.field_type {
            FieldType::Str(ref options) => {
                s.serialize_field("type", "text")?;
                s.serialize_field("options", options)?;
            }
            FieldType::U64(ref options) => {
                s.serialize_field("type", "u64")?;
                s.serialize_field("options", options)?;
            }
            FieldType::I64(ref options) => {
                s.serialize_field("type", "i64")?;
                s.serialize_field("options", options)?;
            }
            FieldType::F64(ref options) => {
                s.serialize_field("type", "f64")?;
                s.serialize_field("options", options)?;
            }
            FieldType::Date(ref options) => {
                s.serialize_field("type", "date")?;
                s.serialize_field("options", options)?;
            }
            FieldType::HierarchicalFacet => {
                s.serialize_field("type", "hierarchical_facet")?;
            }
            FieldType::Bytes => {
                s.serialize_field("type", "bytes")?;
            }
        }

        s.end()
    }
}