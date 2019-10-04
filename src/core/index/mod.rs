use crate::schema::lib::Schema;
use std::sync::Arc;
use crate::tokenizer::manager::TokenizerManager;
use crate::core::index::meta::{SegmentMetaInventory, IndexMeta};
use crate::core::executor::Executor;
use crate::directory::managed::ManagedDirectory;
use crate::directory::lib::Directory;
use crate::Result;
use std::borrow::BorrowMut;

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
    fn create_from_metas(directory: ManagedDirectory,
        metas: &IndexMeta,
        inventory: SegmentMetaInventory) -> Result<Index> {
        let schema =  metas.schema.clone();
        Ok(Index {
            directory,
            schema,
            tokenizers: Default::default(),
            executor: Arc::new(Executor::single_thread()),
            inventory,
        })
    }

}