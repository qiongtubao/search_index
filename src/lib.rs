#[macro_use]
extern crate lazy_static;

use std::io::Write;
mod analyzer;
mod codec;
mod directory;
mod error;
mod reader;
mod schema;
mod serial;
mod writer;
pub type DocId = usize;

pub trait Flushable {
    fn flush<W: Write>(&self, writer: &mut W) -> Result<usize, std::io::Error>;
}

#[cfg(test)]
mod index {
    use crate::directory::Directory;
    use crate::schema::field::Field;
    use crate::schema::Document;
    use crate::serial::SerializableSegment;
    use crate::serial::{DocCursor, TermCursor};
    use crate::writer::closed_index::ClosedIndexWriter;
    use crate::writer::IndexWriter;
    fn show_doc_cursor<'a, D: DocCursor>(mut doc_cursor: D) {
        loop {
            match doc_cursor.next() {
                Some(doc) => {
                    println!("       {}", doc);
                }
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
                doc.set(Field(1), "a b");
                index_writer.add(doc);
            }
            {
                let mut doc = Document::new();
                doc.set(Field(1), "a b c");
                index_writer.add(doc);
            }
            let mut closed_index_writer: ClosedIndexWriter = index_writer.close();
            let mut term_cursor = closed_index_writer.term_cursor();

            loop {
                match term_cursor.next() {
                    Some((term, doc_it)) => {
                        println!("{:?}", term);
                        for doc in doc_it {
                            println!(" doc {}", doc);
                        }
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
