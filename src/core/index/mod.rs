use crate::schema::lib::Schema;
use std::sync::Arc;
use crate::tokenizer::manager::TokenizerManager;
use crate::core::index::meta::SegmentMetaInventory;
use crate::core::executor::Executor;
use crate::directory::managed::ManagedDirectory;
use crate::directory::lib::Directory;
use crate::Result;
pub mod meta;

#[derive(Clone)]
pub struct Index {
    directory: ManagedDirectory,
    schema: Schema,
    executor: Arc<Executor>,
    tokenizers: TokenizerManager,
    inventory: SegmentMetaInventory,
}

impl Index {

}