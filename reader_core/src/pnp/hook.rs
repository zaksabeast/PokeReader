use super::bindings;

pub fn get_trampoline_addr() -> u32 {
    unsafe { bindings::get_trampoline_addr() }
}

pub fn get_route_hook_addr() -> u32 {
    unsafe { bindings::get_route_hook_addr() }
}
