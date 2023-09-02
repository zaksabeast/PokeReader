use linked_list_allocator::LockedHeap;

pub const HEAP_SIZE: usize = 0x100000;

#[repr(align(0x1000))]
struct AlignedStruct {
    data: [u8; HEAP_SIZE],
}

// Function to create and return the global AlignedStruct instance
unsafe fn get_heap_bottom() -> *mut u8 {
    // Declare the global instance as a static variable
    static mut GLOBAL_ALIGNED_STRUCT: AlignedStruct = AlignedStruct {
        data: [0; HEAP_SIZE],
    };

    // Return a reference to the global instance
    GLOBAL_ALIGNED_STRUCT.data.as_mut_ptr()
}

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// # Safety
/// This function should only be used one time.
pub unsafe fn init_heap() {
    ALLOCATOR.lock().init(get_heap_bottom(), HEAP_SIZE)
}
