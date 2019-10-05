use crate::schema::lib::field::entry::FieldEntry;
use std::collections::HashMap;
use crate::schema::lib::field::Field;
use crate::schema::lib::Schema;
use crate::schema::lib::inner::InnerSchema;
use std::sync::Arc;
use crate::schema::lib::field::options::text::TextOptions;
use crate::schema::lib::field::options::int::IntOptions;

#[derive(Default)]
pub struct SchemaBuilder {
    pub fields: Vec<FieldEntry>,
    pub fields_map: HashMap<String, Field>,
}

impl SchemaBuilder {
    pub fn new() -> Self {
        SchemaBuilder::default()
    }
    pub fn add_u64_field<T: Into<IntOptions>>(&mut self, field_name_str: &str, field_options: T,)-> Field {
        let field_name = String::from(field_name_str);
        let field_entry = FieldEntry::new_u64(field_name, field_options.into());
        self.add_field(field_entry)
    }
    pub fn add_text_field<T: Into<TextOptions>>(&mut self, field_name_str: &str, field_options: T) -> Field {
        let field_name = String::from(field_name_str);
        let field_entry = FieldEntry::new_text(field_name, field_options.into());
        self.add_field(field_entry)
    }
    pub fn add_field(&mut self, field_entry: FieldEntry) -> Field {
        let field = Field(self.fields.len() as u32);
        let field_name = field_entry.name().to_string();
        self.fields.push(field_entry);
        self.fields_map.insert(field_name, field);
        field
    }
    pub fn build(self) -> Schema {
        Schema(Arc::new(InnerSchema {
            fields: self.fields,
            fields_map: self.fields_map,
        }))
    }
}

