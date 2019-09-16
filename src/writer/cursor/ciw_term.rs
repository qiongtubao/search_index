use crate::schema::field::Field;
use crate::schema::term::Term;
use crate::serial::{DocCursor, TermCursor};
use crate::writer::cursor::ciw_doc::CIWDocCursor;
use crate::writer::cursor::ciw_form::{CIWFormCursor, FormPostings};
use crate::writer::field::FieldWriter;
use crate::writer::postings::SimplePostingsWriter;
use crate::DocId;
use std::collections::btree_map;
use std::collections::hash_map;

pub struct CIWTermCursor<'a> {
    pub field_it: hash_map::Iter<'a, Field, FieldWriter>,
    pub form_it: CIWFormCursor<'a>,
    pub current_form_postings: Option<FormPostings<'a>>,
    pub field: &'a Field,
}

impl<'a> CIWTermCursor<'a> {
    fn advance(&mut self) -> bool {
        if self.next_form() {
            true
        } else if self.next_field() {
            self.advance()
        } else {
            false
        }
    }

    fn get_term(&self) -> Term<'a> {
        Term {
            field: self.field.clone(),
            text: self.current_form_postings.as_ref().unwrap().form,
        }
    }

    fn doc_cursor(&self) -> CIWDocCursor<'a> {
        let postings = self.current_form_postings.as_ref().unwrap().postings;
        CIWDocCursor {
            docs_it: postings.doc_ids.iter(),
            current: None,
            num_docs: postings.doc_ids.len(),
        }
    }
    fn next_form(&mut self) -> bool {
        match self.form_it.next() {
            Some(form_postings) => {
                self.current_form_postings = Some(form_postings);
                return true;
            }
            None => false,
        }
    }
    fn next_field(&mut self) -> bool {
        match self.field_it.next() {
            Some((filed, field_writer)) => {
                self.form_it = CIWFormCursor {
                    term_it: field_writer.term_index.iter(),
                    postings_map: &field_writer.postings,
                };
                self.field = filed;
                true
            }
            None => false,
        }
    }
}

impl<'a> TermCursor<'a> for CIWTermCursor<'a> {
    type DocCur = CIWDocCursor<'a>;
    fn next(&mut self) -> Option<(Term<'a>, Self::DocCur)> {
        if self.advance() {
            Some((self.get_term(), self.doc_cursor()))
        } else {
            None
        }
    }
}
