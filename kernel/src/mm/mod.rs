mod heap_allocater;
mod address;
mod page_table;
mod frame_allocator;
mod memory_set;


pub use address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
use address::{StepByOne, VPNRange};
pub use frame_allocator::{frame_alloc, FrameTracker};
pub use memory_set::remap_test;
pub use memory_set::{MapPermission, MemorySet, KERNEL_SPACE};
pub use page_table::{translated_byte_buffer, PageTableEntry};
use page_table::{PTEFlags, PageTable};

pub fn init() {
    heap_allocater::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}

pub fn test() {
    heap_allocater::heap_test();
    frame_allocator::frame_allocator_test();
    memory_set::remap_test();
}
