use crate::schema::lib::field::options::int::IntOptions;
use crate::schema::lib::field::r#type::FieldType;
use crate::schema::lib::field::options::text::TextOptions;

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