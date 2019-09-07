use crate::analyzer::tokenize;
use crate::schema::field::Field;
use crate::schema::term::Term;
use crate::schema::Document;
use crate::writer::field::FieldWriter;
use std::collections::HashMap;
use crate::directory::Directory;

mod field;
mod postings;
struct IndexWriter {
    max_doc: usize,
    term_writers: HashMap<Field, FieldWriter>,
    directory: Directory,
}

impl IndexWriter {
    pub fn open(directory: &Directory) -> IndexWriter {
        IndexWriter {
            max_doc: 0,
            term_writers: HashMap::new(),
            directory: (*directory).clone(),
        }
    }
    pub fn get_field_writer(&mut self, field: &Field) -> &mut FieldWriter {
        if !self.term_writers.contains_key(field) {
            self.term_writers
                .insert((*field).clone(), FieldWriter::new());
        }
        self.term_writers.get_mut(field).unwrap()
    }
    pub fn add(&mut self, doc: Document) {
        let doc_id = self.max_doc;
        for field_value in doc {
            let field = field_value.field;
            let field_writer = self.get_field_writer(&field);
            for token in tokenize(&field_value.text) {
                field_writer.suscribe(doc_id, token);
            }
        }
        self.max_doc += 1;
    }
    fn suscribe(&self, term: &Term, doc_id: usize) {}
}

#[cfg(test)]
mod writer {
    use crate::schema::field::Field;
    use crate::schema::Document;
    use crate::writer::IndexWriter;
    use crate::directory::Directory;
    use crate::writer::postings::SimplePostingsWriter;
    use crate::Flushable;
    use crate::writer::postings::PostingsWriter;

    #[test]
    fn add_doc() {
        let directory = Directory::open("toto");
        let mut wirter = IndexWriter::open(&directory);
        let mut doc = Document::new();
        doc.set(Field("text"), &String::from("toto"));
        wirter.add(doc);
    }
    #[test]
    fn test_postings_writer() {
        let mut postings_writer = SimplePostingsWriter::new();
        postings_writer.suscribe(1);
        postings_writer.suscribe(4);
        postings_writer.suscribe(5);
        postings_writer.suscribe(17);
        let mut buffer: Vec<u8> = Vec::new();
        assert_eq!(buffer.len(), 0);
        postings_writer.flush(&mut buffer);
        assert_eq!(buffer.len(), 5 * 8);
    }
}
