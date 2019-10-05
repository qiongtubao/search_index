use crate::schema::lib::field::Field;
use crate::schema::lib::value::Value;
#[derive(Debug)]
pub struct FieldValue {
    field: Field,
    value: Value,
}