use crate::schema::lib::field::value::FieldValue;
#[derive(Debug)]
pub struct Document {
    field_values: Vec<FieldValue>,
}
