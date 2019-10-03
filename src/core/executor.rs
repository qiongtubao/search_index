use scoped_pool::Pool;
pub enum Executor {
    SingleThread,
    ThreadPool(Pool),
}