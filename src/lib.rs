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
    use crate::serial::{SerializableSegment};
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
    fn show_term<'a, T: TermCursor<'a>>(term_cursor: &T) {
        println!("{:?}", term_cursor.get_term());
        let doc_cursor = term_cursor.doc_cursor();
        for doc in doc_cursor {
            println!("doc({})", doc);
        }
    }

    #[test]
    fn test_indexing() {
        let directory = Directory::in_mem();
        {
            let mut index_writer = IndexWriter::open(&directory);
            {
                let mut doc = Document::new();
                doc.set(Field(1), "a b");
                index_writer.add(doc);
            }
            {
                let mut doc = Document::new();
                doc.set(Field(1), "a b c");
                index_writer.add(doc);
            }
            let mut closed_index_writer:  ClosedIndexWriter = index_writer.close();
            let mut term_cursor = closed_index_writer.term_cursor();

            loop {
                if !term_cursor.advance() {
                    break;
                }
                show_term(&term_cursor);
            }
//          assert!(false);

        }
    }
}
