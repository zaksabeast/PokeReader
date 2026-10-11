#![no_std]
#![allow(static_mut_refs)]
// The entrypoint and hooks only exist on the 3DS, so host builds (tests) don't use everything.
#![cfg_attr(not(target_os = "horizon"), allow(dead_code))]

extern crate alloc;

#[cfg(target_os = "horizon")]
mod allocator;

mod app;
mod crystal;
mod ctr;
mod draw;
mod gen6;
mod gen7;
#[cfg(target_os = "horizon")]
mod hook;
mod memory;
mod rng;
mod time;
mod trampoline;
mod transporter;
mod utils;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const GIT_HASH: &str = env!("GIT_HASH");

#[cfg(target_os = "horizon")]
#[panic_handler]
fn my_panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = info.location() {
        let file = location.file();
        let slice = &file[file.len() - 7..];

        // Since we're about to break, storing a few u32s in these registers won't break us further.
        // In the future it might be helpful to disable this for release builds.
        unsafe {
            // r9 and r10 aren't used as frequently as the lower registers, so in most situations
            // we'll get more useful information by storing the last 4 characters of the file name
            // and the line number where we broke.
            let partial_file_name = *(slice.as_ptr() as *const u32);
            core::arch::asm!("mov r9, {}", in(reg) partial_file_name);
            core::arch::asm!("mov r10, {}", in(reg) location.line());
        }
    }

    // svcBreak(USERBREAK_PANIC)
    unsafe { core::arch::asm!("svc 0x3C", in("r0") 0u32) };
    loop {}
}

#[cfg(target_os = "horizon")]
extern "C" {
    static mut fake_heap_start: *mut u8;
    static mut fake_heap_end: *mut u8;

    static mut __ctru_heap: u32;
}

#[cfg(target_os = "horizon")]
#[no_mangle]
pub static mut __ctru_heap_size: u32 = 0;

#[cfg(target_os = "horizon")]
#[no_mangle]
pub static mut __ctru_linear_heap_size: u32 = 0;

#[cfg(target_os = "horizon")]
unsafe fn system_allocate_heaps() {
    let header = ctr::plgldr::get_header();
    __ctru_heap_size = header.heap_size;
    __ctru_heap = header.heap_va;

    // Set up newlib heap.
    fake_heap_start = __ctru_heap as *mut u8;
    fake_heap_end = fake_heap_start.add(__ctru_heap_size as usize);
}

/// Entrypoint, called by 3gx_crt0.s. The game starts when this returns.
#[cfg(target_os = "horizon")]
#[no_mangle]
pub extern "C" fn main() {
    unsafe { system_allocate_heaps() };

    let _ = ctr::srv::init();

    // Acknowledge plugin loader events (sleep, swap, exit) so the system doesn't wait on us.
    if ctr::plgldr::init().is_ok() {
        ctr::plgldr::start_ack_events_thread();
    }

    time::set_game_start_ms();

    let _ = hook::install_hooks();

    // Game hooks need the trampoline written by install_hooks.
    if trampoline::get_trampoline_addr() != 0 {
        app::init();
    }

    ctr::invalidate_entire_instruction_cache();
}
