use crate::writer::IndexWriter;
use crate::schema::field::Field;
use crate::writer::field::FieldWriter;
use std::collections::hash_map;
use crate::serial::SerializableSegment;
//use crate::writer::cursor::ciw_field::CIWFieldCursor;
use crate::writer::cursor::ciw_term::CIWTermCursor;
use crate::writer::cursor::ciw_form::CIWFormCursor;


pub struct ClosedIndexWriter {
    pub index_writer: IndexWriter,
}

impl<'a> SerializableSegment<'a> for ClosedIndexWriter {
    type TermCur = CIWTermCursor<'a>;
    fn term_cursor(&'a self) -> Self::TermCur {
        let mut field_it: hash_map::Iter<'a, Field, FieldWriter> = self.index_writer.term_writers.iter();
        let (field, field_writer) = field_it.next().unwrap();
        CIWTermCursor {
            field_it,
            form_it: CIWFormCursor {
                term_it: field_writer.term_index.iter(),
                postings_map: &field_writer.postings,
            },
            field,
            current_form_postings: None
        }
    }
}