#![no_std]

extern crate alloc;

#[cfg(target_os = "horizon")]
mod allocator;

mod draw;
mod gen6;
mod gen7;
mod menu;
mod pnp;
mod rng;
mod title;
mod transporter;
mod utils;

use title::{title_id, SupportedTitle};

#[cfg(target_os = "horizon")]
#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(target_os = "horizon")]
#[no_mangle]
pub extern "C" fn initialize() {
    unsafe { allocator::init_heap() };
}

#[no_mangle]
pub extern "C" fn run_frame() {
    match title_id() {
        SupportedTitle::S | SupportedTitle::M => gen7::run_sm_frame(),
        SupportedTitle::Us | SupportedTitle::Um => gen7::run_usum_frame(),
        SupportedTitle::Or | SupportedTitle::As => gen6::run_oras_frame(),
        SupportedTitle::X | SupportedTitle::Y => gen6::run_xy_frame(),
        SupportedTitle::Transporter => transporter::run_frame(),
        SupportedTitle::Invalid => {}
    }
}
