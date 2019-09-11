use crate::writer::field::FieldWriter;
use crate::schema::field::Field;
use std::collections::hash_map;
use crate::serial::TermCursor;
use crate::writer::cursor::ciw_term::CIWTermCursor;

pub struct CIWFieldCursor<'a> {
    pub field_it: hash_map::Iter<'a, Field, FieldWriter>,
    pub current: Option<(&'a Field, &'a FieldWriter)>
}
impl<'a> CIWFieldCursor<'a> {
    fn get_field_writer(&self) -> &'a FieldWriter {
        self.current.map(|(_, second)| second).unwrap()
    }
}

impl<'a> Iterator for CIWFieldCursor<'a> {
    type Item=&'a Field;
    fn next(&mut self) -> Option<&'a Field> {
        self.current = self.field_it.next();
        self.get_field()
    }
}

impl<'a> FieldCursor<'a> for CIWFieldCursor<'a> {
    type TTermCur = CIWTermCursor<'a>;
    fn get_field(&self) -> Option<&'a Field> {
        self.current.map(|(first, _)| first)
    }

    fn term_cursor<'b>(&'b self) -> CIWTermCursor<'b>  {
        let field_writer = self.get_field_writer();
        CIWTermCursor {
            postings: &field_writer.postings,
            term_it: field_writer.term_index.iter(),
            current: None
        }
    }
}