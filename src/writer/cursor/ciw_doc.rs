use crate::DocId;
use crate::serial::DocCursor;

pub struct CIWDocCursor<'a> {
    pub docs_it: Box<Iterator<Item=&'a DocId> + 'a>,
    pub current: Option<DocId>,
}
impl<'a> Iterator for CIWDocCursor<'a> {
    type Item = DocId;
    fn next(&mut self) -> Option<DocId> {
        self.current = self.docs_it.next().map(|x| *x);
        self.current
    }
}

impl<'a> DocCursor for CIWDocCursor<'a> {
    fn doc(&self) -> usize {
        self.current.unwrap()
    }
}