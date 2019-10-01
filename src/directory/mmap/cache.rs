use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Weak;
use crate::directory::mmap::BoxedData;

#[derive(Default, Clone, Debug)]
pub struct CacheCounters {
    pub hit: usize,
    pub miss: usize,
}

pub struct MmapCache {
    counters: CacheCounters,
    cache: HashMap<PathBuf, Weak<BoxedData>>,
}

impl Default for MmapCache {
    fn default() -> MmapCache {
        MmapCache {
            counters: Default::default(),
            cache: HashMap::new(),
        }
    }
}