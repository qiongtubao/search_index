use scoped_pool::Pool;
pub enum Executor {
    SingleThread,
    ThreadPool(Pool),
}
impl Executor {
    pub fn single_thread() -> Executor {
        Executor::SingleThread
    }
}