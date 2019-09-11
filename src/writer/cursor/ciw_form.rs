use crate::writer::postings::SimplePostingsWriter;
use std::collections::btree_map;
pub struct FormPostings<'a> {
    pub form: &'a str,
    pub postings: &'a SimplePostingsWriter,
}

pub struct CIWFormCursor<'a> {
    pub term_it: btree_map::Iter<'a, String, usize>,
    pub postings_map: &'a Vec<SimplePostingsWriter>,
}

impl<'a> Iterator for CIWFormCursor<'a> {
    type Item = FormPostings<'a>;
    fn next(&mut self) -> Option<FormPostings<'a>> {
        self.term_it.next().map(|(form, postings_idx)| {
            FormPostings {
                form: form,
                postings: unsafe {self.postings_map.get_unchecked(*postings_idx)}
            }
        })
    }
}

