use crate::{DocId, Flushable};
use std::io::Write;
use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};

pub trait PostingsWriter {
    fn suscribe(&mut self, doc_id: DocId);
}

pub struct SimplePostingsWriter {
    pub doc_ids: Vec<DocId>,
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
        //从小到大排序
        if self.doc_ids.len() == 0 || self.doc_ids[self.doc_ids.len() -1] < doc_id {
            self.doc_ids.push(doc_id);
        }
    }
}


impl Flushable for SimplePostingsWriter {
    fn flush<W: Write>(&self, writer: &mut W) -> Result<usize, std::io::Error> {
        let mut num_bytes_written = 0;
        let num_docs = self.doc_ids.len() as u64;
        writer.write_u64::<NativeEndian>(num_docs);
        num_bytes_written += 8;
        for &doc_id in self.doc_ids.iter() {
            writer.write_u64::<NativeEndian>(doc_id as u64);
            num_bytes_written += 8;
        }
        Ok(num_bytes_written)
    }
}
