use std::sync::{RwLock, Weak, Arc};

pub type WatchCallback = Box<dyn Fn() -> () + Sync + Send>;
#[derive(Default)]
pub struct WatchCallbackList {
    router: RwLock<Vec<Weak<WatchCallback>>>,
}
impl WatchCallbackList {
    pub fn subscribe(&self, watch_callback: WatchCallback) -> WatchHandle {
        let watch_callback_arc = Arc::new(watch_callback);
        //关于weak  => http://wiki.jikexueyuan.com/project/rust-primer/rcarc/rcarc.html
        let watch_callback_weak = Arc::downgrade(&watch_callback_arc);
        {
            self.router.write().unwrap().push(watch_callback_weak);
        }
        WatchHandle(watch_callback_arc)
    }
    fn list_callback(&self) -> Vec<Arc<WatchCallback>> {
        let mut callbacks = vec![];
        let mut router_wlock = self.router.write().unwrap();
        let mut i = 0;
        while i < router_wlock.len() {
            if let Some(watch) = router_wlock[i].upgrade() {
                callbacks.push(watch);
                i += 1;
            } else {
                router_wlock.swap_remove(i);
            }
        }
        callbacks
    }
    pub fn broadcast(&self) {
        let callbacks = self.list_callback();
        let spawn_res = std::thread::Builder::new()
            .name("watch-callbacks".to_string())
            .spawn(move || {
                for callback in callbacks {
                    callback();
                }
            });
        if let Err(err) = spawn_res {
            println!(
                "Failed to spawn thread to call watch callbacks. Cause: {:?}",
                err
            );
        }
    }
}
#[derive(Clone)]
pub struct WatchHandle(Arc<WatchCallback>);


