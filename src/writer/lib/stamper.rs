use std::sync::Arc;
use crate::core::index::meta::Opstamp;
use std::sync::atomic::{Ordering, AtomicU64};

#[derive(Default)]
pub struct Stamper(Arc<AtomicU64>);

impl Stamper {
    //增加
    pub fn stamp(&self) -> Opstamp {
        self.0.fetch_add(1u64, Ordering::SeqCst) as u64
    }
}