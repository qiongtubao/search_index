use crate::writer::IndexWriter;
use crate::schema::field::Field;
use crate::writer::field::FieldWriter;
use std::collections::hash_map;
use crate::serial::{FieldCursor, SerializableSegment};
use crate::writer::cursor::ciw_field::CIWFieldCursor;


pub struct ClosedIndexWriter {
    pub index_writer: IndexWriter,
}

impl<'a> SerializableSegment<'a> for ClosedIndexWriter {
    type TFieldCur = CIWFieldCursor<'a>;
    fn field_cursor(&'a self) -> Self::TFieldCur {
        let mut field_it = self.index_writer.term_writers.iter();
        let current: Option<(&'a Field, &'a FieldWriter)> = None;

            CIWFieldCursor {
                current,
                field_it,
            }
    }
}