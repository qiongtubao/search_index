use crate::writer::postings::SimplePostingsWriter;
use std::collections::btree_map;
use crate::schema::field::Field;
use crate::serial::{TermCursor, DocCursor};
use crate::writer::cursor::ciw_doc::CIWDocCursor;
use crate::DocId;

pub struct CIWTermCursor<'a> {
    pub postings: &'a Vec<SimplePostingsWriter>,
    pub term_it: btree_map::Iter<'a, String, usize>,
    pub current: Option<(&'a String, &'a usize)>
}

impl<'a> CIWTermCursor<'a> {
    fn get_term_option(&self) -> Option<&'a String> {
        self.current
            .map(|(first, _)| first)
    }
}

impl<'a> Iterator for CIWTermCursor<'a> {
    type Item=&'a String;
    fn next(&mut self) -> Option<&'a String> {
        self.current = self.term_it.next();
        self.get_term_option()
    }
}

impl<'a> TermCursor<'a> for CIWTermCursor<'a> {
    type TDocCur = CIWDocCursor<'a>;

    fn get_term(&self) -> &'a String {
        self.get_term_option().unwrap()
    }

    fn doc_cursor(&self) -> CIWDocCursor<'a> {
        let (_, &postings_id) = self.current.unwrap();
        unsafe {
            let postings_writer = self.postings.get_unchecked(postings_id);
            let docs_it = postings_writer.doc_ids.iter();
            return CIWDocCursor {
                    docs_it: Box::new(docs_it),
                    current: None,
                }

        }
    }
}
