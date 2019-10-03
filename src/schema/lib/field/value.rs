use crate::schema::lib::field::Field;
use crate::schema::lib::value::Value;

pub struct FieldValue {
    field: Field,
    value: Value,
}