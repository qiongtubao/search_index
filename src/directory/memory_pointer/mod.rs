
mod resident;
mod memory;
pub trait  MemoryPointer {
    fn len(&self) -> usize;
    fn ptr(&self) -> *const u8;
}