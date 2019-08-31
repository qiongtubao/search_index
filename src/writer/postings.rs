use crate::DocId;

pub struct PostingsWriter {
    doc_ids: Vec<DocId>,
}
impl PostingsWriter {
    pub fn new() -> Self {
        PostingsWriter {
            doc_ids: Vec::new(),
        }
    }
    pub fn suscribe(&mut self, doc_id: DocId) {
        self.doc_ids.push(doc_id);
    }
}
