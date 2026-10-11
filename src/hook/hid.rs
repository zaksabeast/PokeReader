use crate::ctr::hid::set_key_addr;
use crate::ctr::{Handle, map_memory_block};

/// Replaces the game's `svcMapMemoryBlock` call for the HID shared memory,
/// so we can learn where the pad state lives.
pub unsafe extern "C" fn map_input_hook(
    memblock_handle: u32,
    addr: *mut u32,
    _r2: u32,
    _r3: u32,
    _r4: u32,
    r5: u32,
) -> i32 {
    let read_only = r5 != 0;
    if read_only {
        set_key_addr(addr.add(10));
    }

    // Mimic the game's permissions
    let my_perm = match read_only {
        false => 3, // MEMPERM_READ | MEMPERM_WRITE
        true => 1,  // MEMPERM_READ
    };

    let memperm_dontcare = 0x10000000;
    map_memory_block(Handle(memblock_handle), addr as u32, my_perm, memperm_dontcare)
        .map_or_else(|err| err, |_| 0)
}
