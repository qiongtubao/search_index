use crate::core::index::Index;
use crate::directory::lib::lock::DirectoryLock;
use crate::core::index::meta::Opstamp;
use crate::schema::document::Document;
use crate::writer::lib::stamper::Stamper;
use crate::writer::lib::operation::add::AddOperation;
use crossbeam::channel;
use smallvec::smallvec;
use crate::writer::lib::operation::{OperationSender, OperationReceiver};

mod lib;
pub struct IndexWriter {
    stamper: Stamper, //计数器
    operation_sender: OperationSender,
}
const PIPELINE_MAX_SIZE_IN_DOCS: usize = 10_000;
impl IndexWriter {
    pub fn new(index: &Index,
               num_threads: usize,
               heap_size_in_bytes_per_thread: usize,
               directory_lock: DirectoryLock,) -> crate::Result<Self> {
        let (document_sender, document_receiver): (OperationSender, OperationReceiver) =
            channel::bounded(PIPELINE_MAX_SIZE_IN_DOCS);
        Ok(IndexWriter {
            stamper: Default::default(),
            operation_sender: document_sender
        })
    }
    pub fn add_document(&self, document: Document) -> Opstamp {
        //计数器+1
        let opstamp = self.stamper.stamp();
        let add_operation = AddOperation{ opstamp, document };
        let send_result = self.operation_sender.send(smallvec![add_operation]);
        if let Err(e) = send_result {
            panic!("Failed to index document. Sending to indexing channel failed. This probably means all of the indexing threads have panicked. {:?}", e);
        }
        opstamp
    }
}