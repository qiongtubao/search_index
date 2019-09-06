use memmap::Mmap;
use crate::directory::memory_pointer::MemoryPointer;

struct MmapMemory(Mmap);
impl MemoryPointer for MmapMemory{
    fn len(&self) -> usize {
        let &MmapMemory(ref mmap) = self;
        mmap.len()
    }
    fn ptr(&self) -> *const u8 {
        let &MmapMemory(ref mmap) = self;
        mmap.ptr()
    }
}