use crate::{pnp, title::SupportedTitle};
use core::slice;

static mut MAIN_RNG_SEED_TICKS: u32 = 0;
static mut MAIN_RNG_MS_EPOCH_LOW: u32 = 0;
static mut MAIN_RNG_MS_EPOCH_HIGH: u32 = 0;
static mut SOS_SEED: u32 = 0;

pub fn sos_seed() -> u32 {
    unsafe { SOS_SEED }
}

fn init_sfmt_hook(regs: &[u32]) {
    if regs[0] == 0x30038e30 {
        unsafe { SOS_SEED = regs[1] };
    }
}

fn set_main_rng_seed_ticks(ticks: u32) {
    unsafe {
        if MAIN_RNG_SEED_TICKS == 0 {
            MAIN_RNG_SEED_TICKS = ticks;
        }
    }
}

pub fn main_rng_seed_ticks() -> u32 {
    unsafe { MAIN_RNG_SEED_TICKS }
}

fn set_main_rng_ms_epoch(ms_epoch_high: u32, ms_epoch_low: u32) {
    unsafe {
        MAIN_RNG_MS_EPOCH_HIGH = ms_epoch_high;
        MAIN_RNG_MS_EPOCH_LOW = ms_epoch_low;
    }
}

pub fn main_rng_ms_epoch() -> (u32, u32) {
    unsafe { (MAIN_RNG_MS_EPOCH_HIGH, MAIN_RNG_MS_EPOCH_LOW) }
}

pub fn init_usum() {
    // sos seed
    pnp::write(0x361e60, &0xeb09685du32); // bl 0x5bbfdc

    // main rng seed
    pnp::write(0x3fcbc0, &0xeb06fd05u32); // bl 0x5bbfdc

    // hook router
    pnp::write(0x5bbfdc, &0xe92d4000u32); // stmdb // sp!,{lr}
    pnp::write(0x5bbfe0, &0xe92d5fffu32); // stmdb // sp!,{r0,r1,r2,r3,r4,r5,r6,r7,r8,r9,r10,r11,r12
    pnp::write(0x5bbfe4, &0xe1a0000du32); // mov r0, sp
    pnp::write(0x5bbfe8, &0xe59fe008u32); // ldr // lr,[0x5bbff8]
    pnp::write(0x5bbfec, &0xe59ff008u32); // ldr // pc,[0x5bbffc]
    pnp::write(0x5bbff0, &0xe8bd5fffu32); // ldmia // sp!,{r0,r1,r2,r3,r4,r5,r6,r7,r8,r9,r10,r11,r12
    pnp::write(0x5bbff4, &0xe8bd8000u32); // ldmia // sp!,{pc}
    pnp::write(0x5bbff8, &0x005bbff0u32); // 5BBFF0h
    pnp::write(0x5bbffc, &(route_usum_hook as u32));
}

pub extern "C" fn route_usum_hook(stack_pointer: *mut u32) {
    let regs = unsafe { slice::from_raw_parts_mut(stack_pointer, 15) };
    if regs[13] == (0x361e60 + 4) {
        init_sfmt_hook(regs);
        regs[14] = 0x5609b8;
    } else if regs[13] == (0x3fcbc0 + 4) {
        let ticks = unsafe { stack_pointer.add(4 + 15).read() };
        let ms_epoch_low = unsafe { stack_pointer.add(12 + 15).read() };
        let ms_epoch_high = unsafe { stack_pointer.add(13 + 15).read() };
        set_main_rng_seed_ticks(ticks);
        set_main_rng_ms_epoch(ms_epoch_high, ms_epoch_low);
        regs[14] = 0x4b9b44;
    }
}

pub fn init_sm() {
    // sos seed
    pnp::write(0x359784, &0xeb08fa14u32); // bl 0x597fdc

    // main rng seed
    pnp::write(0x3eab60, &0xeb06b51du32); // bl 0x597fdc

    // hook router
    pnp::write(0x597fdc, &0xe92d4000u32); // stmdb // sp!,{lr}
    pnp::write(0x597fe0, &0xe92d5fffu32); // stmdb // sp!,{r0,r1,r2,r3,r4,r5,r6,r7,r8,r9,r10,r11,r12
    pnp::write(0x597fe4, &0xe1a0000du32); // mov r0, sp
    pnp::write(0x597fe8, &0xe59fe008u32); // ldr // lr,[0x5bbff8]
    pnp::write(0x597fec, &0xe59ff008u32); // ldr // pc,[0x5bbffc]
    pnp::write(0x597ff0, &0xe8bd5fffu32); // ldmia // sp!,{r0,r1,r2,r3,r4,r5,r6,r7,r8,r9,r10,r11,r12
    pnp::write(0x597ff4, &0xe8bd8000u32); // ldmia // sp!,{pc}
    pnp::write(0x597ff8, &0x00597ff0u32); // 597ff0h
    pnp::write(0x597ffc, &(route_sm_hook as u32));
}

pub extern "C" fn route_sm_hook(stack_pointer: *mut u32) {
    let regs = unsafe { slice::from_raw_parts_mut(stack_pointer, 15) };
    if regs[13] == (0x359784 + 4) {
        init_sfmt_hook(regs);
        regs[14] = 0x53d070;
    } else if regs[13] == (0x3eab60 + 4) {
        let ticks = unsafe { stack_pointer.add(4 + 15).read() };
        let ms_epoch_low = unsafe { stack_pointer.add(12 + 15).read() };
        let ms_epoch_high = unsafe { stack_pointer.add(13 + 15).read() };
        set_main_rng_seed_ticks(ticks);
        set_main_rng_ms_epoch(ms_epoch_high, ms_epoch_low);
        regs[14] = 0x49db08;
    }
}
