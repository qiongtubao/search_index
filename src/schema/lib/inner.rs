use std::collections::HashMap;
use crate::schema::lib::field::Field;
use crate::schema::lib::field::entry::FieldEntry;

pub struct InnerSchema {
    //存放属性
    pub fields: Vec<FieldEntry>,
    //对应fields kv
    pub fields_map: HashMap<String, Field>, // transient
}