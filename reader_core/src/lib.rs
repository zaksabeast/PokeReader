#![no_std]
#![allow(static_mut_refs)]

extern crate alloc;

#[cfg(target_os = "horizon")]
mod allocator;

mod crystal;
mod draw;
mod gen6;
mod gen7;
mod pnp;
mod rng;
mod title;
mod transporter;
mod utils;

use title::{supported_title, SupportedTitle};

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
#[no_mangle]
pub extern "C" fn initialize() {
    match supported_title() {
        SupportedTitle::S | SupportedTitle::M => gen7::init_sm(),
        SupportedTitle::Us => gen7::init_us(),
        SupportedTitle::Um => gen7::init_um(),
        SupportedTitle::Or | SupportedTitle::As => gen6::init_oras(),
        SupportedTitle::X | SupportedTitle::Y => gen6::init_xy(),
        SupportedTitle::Transporter => transporter::init_transporter(),
        SupportedTitle::CrystalEn
        | SupportedTitle::CrystalDe
        | SupportedTitle::CrystalFr
        | SupportedTitle::CrystalEs
        | SupportedTitle::CrystalIt => crystal::init_crystal(),
        SupportedTitle::Invalid => {}
    }
}

#[no_mangle]
pub extern "C" fn run_frame() {
    match supported_title() {
        SupportedTitle::S | SupportedTitle::M => gen7::run_sm_frame(),
        SupportedTitle::Us | SupportedTitle::Um => gen7::run_usum_frame(),
        SupportedTitle::Or | SupportedTitle::As => gen6::run_oras_frame(),
        SupportedTitle::X | SupportedTitle::Y => gen6::run_xy_frame(),
        SupportedTitle::Transporter => transporter::run_frame(),
        SupportedTitle::CrystalEn
        | SupportedTitle::CrystalDe
        | SupportedTitle::CrystalFr
        | SupportedTitle::CrystalEs
        | SupportedTitle::CrystalIt => crystal::run_frame(),
        SupportedTitle::Invalid => {
            pnp::println!("Unsupported game update!");
            pnp::println!("");
            pnp::println!("Please update your game");
            pnp::println!("for PokeReader to run");
        }
    }
}
