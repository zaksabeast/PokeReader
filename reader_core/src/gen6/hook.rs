use crate::{pnp, utils};

static mut TINYMT_SEED: u32 = 0;

pub fn tinymt_seed() -> u32 {
    unsafe { TINYMT_SEED }
}

fn init_tinymt_hook(regs: &[u32], _stack_pointer: *mut u32) {
    if regs[0] == 0x8c52808 || regs[0] == 0x8c59e04 {
        unsafe { TINYMT_SEED = regs[1] };
    }
}

pub fn init_xy() {
    todo!("Update the hook_game_branch to live within reader's allocated memory")
    // utils::hook_game_branch! {
    //     hook_router_addr = 0x0,
    //     init_tinymt_hook = 0x11e880,
    // }
}

pub fn init_oras() {
    todo!("Update the hook_game_branch to live within reader's allocated memory")
    // utils::hook_game_branch! {
    //     hook_router_addr = 0x0,
    //     init_tinymt_hook = 0x1122fc,
    // }
}
