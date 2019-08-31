use crate::schema::field_value::FieldValue;
use crate::schema::field::Field;
pub mod term;
mod field;
mod field_value;
/*
    Document 数据存储在fields下面
*/
pub struct Document {
    fields: Vec<FieldValue>,
}

impl Document {
    pub fn new() -> Document {
        Document {
            fields: Vec::new(),
        }
    }
    pub fn set(&mut self, field: &Field, text:&String) {
        self.add(FieldValue {
            field: (*field).clone(),
            text: (*text).clone()
        });
    }
    pub fn add(&mut self, field_value: FieldValue) {
        self.fields.push(field_value);
    }
}

impl IntoIterator for Document {
    type Item = FieldValue;
    type IntoIter = std::vec::IntoIter<FieldValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.fields.into_iter()
    }
}