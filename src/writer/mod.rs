use crate::schema::Document;
use crate::analyzer::tokenize;
use crate::schema::term::Term;

struct IndexWriter {
    max_doc: usize,
}
impl IndexWriter {
    pub fn add(&mut self, doc: Document) {
        let doc_id = self.max_doc;
        for field_value in doc {
            for token in tokenize(&field_value.text) {
                let term = Term {
                    field: &field_value.field,
                    text: &token
                };
                self.suscribe(&term, doc_id);
            }
        }
        self.max_doc += 1;
    }
    fn suscribe(&self, term: &Term, doc_id: usize) {

    }
}

#[cfg(test)]
mod writer {
    use crate::writer::IndexWriter;
    use crate::schema::Document;

    #[test]
    fn add_doc() {
        let mut wirter = IndexWriter {
            max_doc: 0
        };
        let doc = Document::new();
        wirter.add(doc);
    }
}