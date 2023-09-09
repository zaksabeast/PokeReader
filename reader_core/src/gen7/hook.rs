use crate::utils;

static mut MAIN_RNG_SEED_TICKS: u32 = 0;
static mut MAIN_RNG_MS_EPOCH_LOW: u32 = 0;
static mut MAIN_RNG_MS_EPOCH_HIGH: u32 = 0;
static mut SOS_SEED: u32 = 0;

pub fn sos_seed() -> u32 {
    unsafe { SOS_SEED }
}

fn init_sfmt_hook(regs: &[u32], _stack_pointer: *mut u32) {
    if regs[0] == 0x30038e30 {
        unsafe { SOS_SEED = regs[1] };
    }
}

pub struct RngSeedContext {
    pub epoch_high: u32,
    pub epoch_low: u32,
    pub ticks: u32,
}

pub fn main_rng_seed_context() -> RngSeedContext {
    unsafe {
        RngSeedContext {
            epoch_high: MAIN_RNG_MS_EPOCH_HIGH,
            epoch_low: MAIN_RNG_MS_EPOCH_LOW,
            ticks: MAIN_RNG_SEED_TICKS,
        }
    }
}

fn init_main_rng_hook(_regs: &[u32], stack_pointer: *mut u32) {
    let ticks = unsafe { stack_pointer.add(4).read() };
    let ms_epoch_low = unsafe { stack_pointer.add(12).read() };
    let ms_epoch_high = unsafe { stack_pointer.add(13).read() };
    unsafe {
        MAIN_RNG_SEED_TICKS = ticks;
        MAIN_RNG_MS_EPOCH_HIGH = ms_epoch_high;
        MAIN_RNG_MS_EPOCH_LOW = ms_epoch_low;
    }
}

pub fn init_usum() {
    utils::hook_game_branch! {
        hook_router_addr = 0x5bbfdc,
        init_sfmt_hook = 0x361e60,
        init_main_rng_hook = 0x3fcbc0,
    }
}

pub fn init_sm() {
    utils::hook_game_branch! {
        hook_router_addr = 0x597fdc,
        init_sfmt_hook = 0x359784,
        init_main_rng_hook = 0x3eab60,
    }
}
