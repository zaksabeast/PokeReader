mod draw;
mod gen6;
mod gen7;
mod menu;
mod rng;
mod title;
mod transporter;
mod utils;

use title::{title_id, SupportedTitle};

// We could use the default allocator,
// but this makes the module more efficient.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
