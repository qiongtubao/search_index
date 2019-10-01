pub mod field;
use std::sync::Arc;


pub mod inner;
use inner::InnerSchema;
use crate::schema::lib::build::SchemaBuilder;

pub mod build;
#[derive(Clone)]
pub struct Schema(Arc<InnerSchema>);
impl Schema {
    pub fn builder() -> SchemaBuilder {
        SchemaBuilder::default()
    }
}