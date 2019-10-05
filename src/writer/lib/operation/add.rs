use crate::core::index::meta::Opstamp;
use crate::schema::document::Document;

#[derive(Debug)]
pub struct AddOperation {
    pub opstamp: Opstamp,
    pub document: Document,
}