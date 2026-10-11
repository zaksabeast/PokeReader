static mut TRAMPOLINE_ADDR: u32 = 0;
static mut ROUTE_HOOK_ADDR: u32 = 0;

/// Set by the hook installer once the trampoline has been written into the game.
pub fn set_trampoline_addrs(trampoline_addr: u32, route_hook_addr: u32) {
    unsafe {
        TRAMPOLINE_ADDR = trampoline_addr;
        ROUTE_HOOK_ADDR = route_hook_addr;
    }
}

pub fn get_trampoline_addr() -> u32 {
    unsafe { TRAMPOLINE_ADDR }
}

pub fn get_route_hook_addr() -> u32 {
    unsafe { ROUTE_HOOK_ADDR }
}
