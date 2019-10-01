use std::sync::{Arc, RwLock};
use std::path::Path;
use crate::directory::error::OpenDirectoryError;
use std::thread;
use crate::directory::watch_event_router::{WatchHandle, WatchCallback};
use crate::directory::mmap::inner_watcher_wrapper::InnerWatcherWrapper;
use crate::directory::META_FILEPATH;
use notify;
#[derive(Clone)]
pub(crate) struct WatcherWrapper {
    inner: Arc<InnerWatcherWrapper>,
}
/**
    暂时理解上是为了监听meta.json
*/
impl WatcherWrapper {
    pub fn new(path: &Path) -> Result<Self, OpenDirectoryError> {
        let (inner, watcher_recv) = InnerWatcherWrapper::new(path).map_err(|err| match err {
            notify::Error::PathNotFound => OpenDirectoryError::DoesNotExist(path.to_owned()),
            _ => {
                panic!("Unknown error while starting watching directory {:?}", path);
            }
        })?;
        let watcher_wrapper = WatcherWrapper {
            inner: Arc::new(inner),
        };
        let watcher_wrapper_clone = watcher_wrapper.clone();
        thread::Builder::new().name("meta-file-watch-thread".to_string()).spawn(move || {
            loop {
                match watcher_recv.recv().map(|evt| evt.path) {
                    Ok(Some(changed_path)) => {
                        // ... Actually subject to false positive.
                        // We might want to be more accurate than this at one point.
                        if let Some(filename) = changed_path.file_name() {
                            if filename == *META_FILEPATH {
                                watcher_wrapper_clone.inner.watcher_router.broadcast();
                            }
                        }
                    },
                    Ok(None) => {

                    },
                    Err(_e) => {
                        break;
                    }
                }

            }
        }).expect("Failed to spawn thread to watch meta.json");
        Ok(watcher_wrapper)
    }
    pub fn watch(&mut self, watch_callback: WatchCallback) -> WatchHandle {
        self.inner.watcher_router.subscribe(watch_callback)
    }

}