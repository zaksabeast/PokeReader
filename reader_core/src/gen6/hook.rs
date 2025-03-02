use crate::utils;

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
    utils::hook_gen6_mt_seed(0x1254f8);
    utils::hook_game_branch! {
        game_name = xy,
        init_tinymt_hook = 0x11e880,
    }
}

pub fn init_oras() {
    utils::hook_gen6_mt_seed(0x125ec8);
    utils::hook_game_branch! {
        game_name = oras,
        init_tinymt_hook = 0x1122fc,
    }
}
