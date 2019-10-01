use crate::directory::watch_event_router::WatchCallbackList;
use std::sync::Mutex;
use std::sync::mpsc::{Receiver, channel, Sender};
use std::path::Path;
use notify::{RawEvent, RecursiveMode, FsEventWatcher, Watcher};

pub(crate) struct InnerWatcherWrapper {
    _watcher: Mutex<notify::RecommendedWatcher>,
    pub watcher_router: WatchCallbackList,
}

impl InnerWatcherWrapper {
    pub fn new(path: &Path) -> Result<(Self, Receiver<notify::RawEvent>), notify::Error> {
        let (tx, watcher_recv): (Sender<RawEvent>, Receiver<RawEvent>) = channel();
        let mut watcher = notify::raw_watcher(tx)?;
        watcher.watch(path, RecursiveMode::Recursive)?;
        let inner = InnerWatcherWrapper {
            _watcher: Mutex::new(watcher),
            watcher_router: Default::default(),
        };
        Ok((inner, watcher_recv))
    }

}