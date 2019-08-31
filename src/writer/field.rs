use crate::writer::postings::PostingsWriter;
use crate::DocId;
use std::collections::BTreeMap;

pub struct FieldWriter {
    postings: Vec<PostingsWriter>,
    term_index: BTreeMap<String, usize>,
}
impl FieldWriter {
    pub fn new() -> FieldWriter {
        FieldWriter {
            term_index: BTreeMap::new(),
            postings: Vec::new(),
        }
    }
    pub fn get_postings_writer(&mut self, term_text: &str) -> &mut PostingsWriter {
        match self.term_index.get(term_text) {
            Some(unord_id) => {
                return &mut self.postings[*unord_id];
            }
            None => {}
        }
        let unnord_id = self.term_index.len();
        self.postings.push(PostingsWriter::new());
        self.term_index.insert(String::from(term_text), unnord_id);
        &mut self.postings[unnord_id]
    }
    pub fn suscribe(&mut self, doc: DocId, term_text: &str) {
        self.get_postings_writer(term_text).suscribe(doc);
    }
}
