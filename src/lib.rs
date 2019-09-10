#[macro_use]
extern crate lazy_static;

use std::io::Write;
mod serial;
mod analyzer;
mod directory;
mod schema;
mod writer;
mod reader;
pub type DocId = usize;


pub trait Flushable {
    fn flush<W: Write>(&self, writer: &mut W) -> Result<usize, std::io::Error>;
}

#[cfg(test)]
mod index {
    use crate::directory::Directory;
    use crate::schema::Document;
    use crate::schema::field::Field;
    use crate::writer::IndexWriter;
    use crate::serial::{TermCursor, DocCursor};
    use crate::writer::closed_index::ClosedIndexWriter;
    use crate::serial::{SerializableSegment,FieldCursor};
    fn show_doc_cursor<'a, D: DocCursor>(mut doc_cursor: D) {
        loop {
            match doc_cursor.next() {
                Some(doc) => {
                    println!("       {}", doc);
                },
                None =>  {
                    break;
                }
            }
        }
    }
        fn show_term_cursor<'a, T: TermCursor<'a>>(mut term_cursor: T) {
        loop {
            match term_cursor.next() {
                Some(term) => {
                    println!(" {:?}", term);
                    show_doc_cursor(term_cursor.doc_cursor());
                },
                None => {
                    break;
                }
            }
        }
    }
    #[test]
    fn test_indexing() {
        let directory = Directory::in_mem();
        {
            let mut index_writer = IndexWriter::open(&directory);
            {
                let mut doc = Document::new();
                doc.set(Field("text"), "toto titi");
                index_writer.add(doc);
            }
            {
                let mut doc = Document::new();
                doc.set(Field("text"), "titi tata");
                index_writer.add(doc);
            }
            let closed_index_writer:  ClosedIndexWriter = index_writer.close();
            let mut field_cursor = closed_index_writer.field_cursor();

            loop {
                match field_cursor.next() {
                    Some(field) => {
                        println!(" {:?}", field);
                        show_term_cursor(field_cursor.term_cursor());
                    }
                    None => {
                        break;
                    }
                }
            }
//          assert!(false);

        }
    }
}
