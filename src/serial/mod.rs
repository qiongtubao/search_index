use crate::DocId;
use crate::schema::field::Field;
use crate::schema::term::Term;

pub trait DocCursor: Iterator<Item=DocId> {
    fn doc(&self) -> DocId;
    fn len(&self) -> usize;
}

pub trait TermCursor<'a>{
    type DocCur: DocCursor;
    fn advance(&mut self) -> bool;
    fn get_term(&self) -> Term<'a>;
    fn doc_cursor(&self) -> Self::DocCur;
}

//pub trait FieldCursor<'a>: Iterator<Item=&'a Field> {
//    type TTermCur: TermCursor<'a>;
//    fn get_field(&self) -> Option<&'a Field>;
//    fn term_cursor(&'a self) -> Self::TTermCur;
//}

pub trait SerializableSegment<'a> {
    type TermCur: TermCursor<'a>; // TODO rename TermCursorImpl
    fn term_cursor(&'a self) -> Self::TermCur;
}
