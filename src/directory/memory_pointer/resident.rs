use crate::directory::memory_pointer::MemoryPointer;

pub struct ResidentMemoryPointer {
    data: Box<[u8]>,
    len: usize
}
impl MemoryPointer for ResidentMemoryPointer {
    fn len(&self) -> usize {
        self.len
    }
    fn ptr(&self) -> *const u8 {
        &self.data[0]
    }
}