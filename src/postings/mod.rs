use crate::DocId;
mod intersection;
mod simple;
mod vec;
pub trait Postings {
    type IteratorType: Iterator<Item = DocId>;
    fn iter(&self) -> Self::IteratorType;
}
