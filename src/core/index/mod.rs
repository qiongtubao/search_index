use crate::schema::lib::Schema;
use std::sync::Arc;
use crate::tokenizer::manager::TokenizerManager;
use crate::core::index::meta::{SegmentMetaInventory, IndexMeta};
use crate::core::executor::Executor;
use crate::directory::managed::ManagedDirectory;
use crate::directory::lib::Directory;
use crate::Result;
use std::borrow::BorrowMut;
use crate::core::segment::updater::save_new_metas;
use crate::directory::META_FILEPATH;
use crate::error::{DataCorruption, TantivyError};
use crate::writer::IndexWriter;
use crate::directory::error::INDEX_WRITER_LOCK;


pub mod meta;
pub mod untracked_index_meta;
#[derive(Clone)]
pub struct Index {
    directory: ManagedDirectory,
    schema: Schema,
    executor: Arc<Executor>,
    tokenizers: TokenizerManager,
    inventory: SegmentMetaInventory,
}

fn load_metas(directory: &dyn Directory, inventory: &SegmentMetaInventory) -> Result<IndexMeta> {
    let meta_data = directory.atomic_read(&META_FILEPATH)?;
    let meta_string = String::from_utf8_lossy(&meta_data);
    IndexMeta::deserialize(&meta_string, &inventory).map_err(|e| {
        DataCorruption::new(
            META_FILEPATH.to_path_buf(),
            format!("Meta file cannot be deserialized. {:?}.", e)
        )
    }).map_err(From::from)

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
    fn from_directory(mut directory: ManagedDirectory, schema: Schema) -> Result<Index> {
        //保存meta信息
        save_new_metas(schema.clone(), directory.borrow_mut())?;
        let metas = IndexMeta::with_schema(schema);
        Index::create_from_metas(directory, &metas, Default::default())
    }
    pub fn create<Dir: Directory>(dir: Dir, schema: Schema) -> Result<Index> {
        //创建managed文件夹
        let directory = ManagedDirectory::wrap(dir)?;
        Index::from_directory(directory, schema)
    }
    pub fn open<D: Directory>(directory: D) -> Result<Index> {
        let directory = ManagedDirectory::wrap(directory)?;
        let inventory = Default::default();
        let metas = load_metas(&directory, &inventory)?;
        Index::create_from_metas(directory, &metas, inventory)
    }
    pub fn writer_with_num_threads (
        &self,
        num_threads: usize,
        overall_heap_size_in_bytes: usize,
    ) -> Result<IndexWriter> {
        let directory_lock = self.directory.acquire_lock(&INDEX_WRITER_LOCK).map_err(|err| {
            TantivyError::LockFailure(
                err, Some("Failed to acquire index lock. If you are using\
                         a regular directory, this means there is already an \
                         `IndexWriter` working on this `Directory`, in this process \
                         or in a different process."
                              .to_string(),)
            )
        })?;
        let heap_size_in_bytes_per_thread = overall_heap_size_in_bytes / num_threads;
        IndexWriter::new(
            self,
            num_threads,
            heap_size_in_bytes_per_thread,
            directory_lock,
        )
    }
}