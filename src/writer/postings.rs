use crate::{DocId, Flushable};
use std::io::Write;
use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};

pub trait PostingsWriter {
    fn suscribe(&mut self, doc_id: DocId);
}

pub struct SimplePostingsWriter {
    doc_ids: Vec<DocId>,
}
impl SimplePostingsWriter {
    pub fn new() -> Self {
        SimplePostingsWriter {
            doc_ids: Vec::new(),
        }
    }
}

impl PostingsWriter for SimplePostingsWriter {
    fn suscribe(&mut self, doc_id: DocId) {
        self.doc_ids.push(doc_id);
    }
}


impl Flushable for SimplePostingsWriter {
    fn flush<W: Write>(&self, writer: &mut W) -> Result<usize, std::io::Error> {
        let num_docs = self.doc_ids.len() as u64;
        writer.write_u64::<NativeEndian>(num_docs);
        for &doc_id in self.doc_ids.iter() {
            writer.write_u64::<NativeEndian>(doc_id as u64);
        }
        Ok(1)
    }
}