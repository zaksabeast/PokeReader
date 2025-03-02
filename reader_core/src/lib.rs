#![no_std]
#![allow(static_mut_refs)]
#![feature(naked_functions)]

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

use title::{loaded_title, LoadedTitle, TitleError};

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

fn initialize_loaded_title(title: LoadedTitle) {
    match title {
        LoadedTitle::S | LoadedTitle::M => gen7::init_sm(),
        LoadedTitle::Us => gen7::init_us(),
        LoadedTitle::Um => gen7::init_um(),
        LoadedTitle::Or | LoadedTitle::As => gen6::init_oras(),
        LoadedTitle::X | LoadedTitle::Y => gen6::init_xy(),
        LoadedTitle::Transporter => transporter::init_transporter(),
        LoadedTitle::CrystalEn
        | LoadedTitle::CrystalDe
        | LoadedTitle::CrystalFr
        | LoadedTitle::CrystalEs
        | LoadedTitle::CrystalIt => crystal::init_crystal(),
    }
}

#[cfg(target_os = "horizon")]
#[no_mangle]
pub extern "C" fn initialize() {
    match loaded_title() {
        Ok(title) => initialize_loaded_title(title),
        Err(_) => {}
    }
}

fn run_loaded_title_frame(title: LoadedTitle) {
    match title {
        LoadedTitle::S | LoadedTitle::M => gen7::run_sm_frame(),
        LoadedTitle::Us | LoadedTitle::Um => gen7::run_usum_frame(),
        LoadedTitle::Or | LoadedTitle::As => gen6::run_oras_frame(),
        LoadedTitle::X | LoadedTitle::Y => gen6::run_xy_frame(),
        LoadedTitle::Transporter => transporter::run_frame(),
        LoadedTitle::CrystalEn
        | LoadedTitle::CrystalDe
        | LoadedTitle::CrystalFr
        | LoadedTitle::CrystalEs
        | LoadedTitle::CrystalIt => crystal::run_frame(),
    }
}

#[no_mangle]
pub extern "C" fn run_frame() {
    match loaded_title() {
        Ok(title) => run_loaded_title_frame(title),
        Err(TitleError::InvalidUpdate) => {
            pnp::println!("Unsupported game update!");
            pnp::println!("");
            pnp::println!("Please update your game");
            pnp::println!("for PokeReader to run");
        }
        Err(TitleError::InvalidTitle) => {}
    }
}
