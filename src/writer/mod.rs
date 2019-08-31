use crate::analyzer::tokenize;
use crate::schema::field::Field;
use crate::schema::term::Term;
use crate::schema::Document;
use crate::writer::field::FieldWriter;
use std::collections::HashMap;
mod field;
mod postings;
struct IndexWriter {
    max_doc: usize,
    term_writers: HashMap<Field, FieldWriter>,
}

impl IndexWriter {
    pub fn new() -> IndexWriter {
        IndexWriter {
            max_doc: 0,
            term_writers: HashMap::new(),
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

    #[test]
    fn add_doc() {
        let mut wirter = IndexWriter::new();
        let mut doc = Document::new();
        doc.set(Field("text"), &String::from("toto"));
        wirter.add(doc);
    }
}
