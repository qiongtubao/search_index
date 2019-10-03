use std::path::Path;

pub mod lib;
pub mod error;
pub mod mmap;
pub mod watch_event_router;
pub mod managed;
use once_cell::sync::Lazy;

pub static MANAGED_FILEPATH: Lazy<&'static Path> = Lazy::new(|| Path::new(".managed.json"));
pub static META_FILEPATH: Lazy<&'static Path> = Lazy::new(|| Path::new("meta.json"));

