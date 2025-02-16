use super::reader::Gen2Reader;
use crate::utils;

const DIV_INCREMENTS: [u8; 16] = [
    0x12, 0x12, 0x12, 0x13, 0x12, 0x12, 0x13, 0x12, 0x12, 0x13, 0x12, 0x12, 0x13, 0x12, 0x12, 0x13,
];

// Don't worry, I don't feel great about this either
// This is hacky while explorations are happening
static mut RNG_ADVANCE: u32 = 0;
static mut ADIV: u8 = 0;
static mut SDIV: u8 = 0;
static mut CYCLE_COUNTER: u32 = 0;

pub fn measured_div() -> u16 {
    unsafe { (ADIV as u16) << 8 | SDIV as u16 }
}

pub fn rng_advance() -> u32 {
    unsafe { RNG_ADVANCE }
}

pub fn reset_rng_advance() {
    unsafe { RNG_ADVANCE = 0 };
}

// This isn't currently used, but it's been helpful
fn update_cycle_counter(regs: &[u32], _stack_pointer: *mut u32) {
    let cycle_counter = regs[0];
    unsafe { CYCLE_COUNTER = CYCLE_COUNTER.wrapping_add(cycle_counter) };
}

#[repr(C)]
pub struct DivTracker {
    last_div: u8,
    index: usize,
    correct_index: bool,
}

impl DivTracker {
    const fn new() -> Self {
        Self {
            last_div: 0,
            index: 0,
            correct_index: false,
        }
    }

    fn update(&mut self, div: u8) {
        let small_index = self.index % DIV_INCREMENTS.len();
        let diff = div.wrapping_sub(self.last_div);
        self.last_div = div;

        if diff != 0x12 && diff != 0x13 {
            self.correct_index = false;
        }

        if diff != DIV_INCREMENTS[small_index]
            && [2, 3, 5, 6, 8, 9].contains(&(small_index))
            && (self.index >= DIV_INCREMENTS.len() || self.correct_index)
        {
            self.index = match small_index {
                2 => 1 + 0x562,
                3 => 1 + 0x563,
                5 => 1 + 0x22b5,
                6 => 1 + 0x22b6,
                8 => 1 + 8,
                9 => 1 + 9,
                _ => 0,
            };
            self.correct_index = true;
        } else if diff != DIV_INCREMENTS[small_index] {
            self.index = 0;
            self.correct_index = false;
        } else {
            self.index = (self.index + 1) % 0x4000;
        }
    }

    pub fn index(&self) -> Option<usize> {
        // Hides until ready
        match self.correct_index {
            true => Some(self.index),
            false => None,
        }
    }
}

static mut ADD_DIV_TRACKER: DivTracker = DivTracker::new();
static mut SUB_DIV_TRACKER: DivTracker = DivTracker::new();

pub fn add_div_tracker() -> &'static DivTracker {
    unsafe { &ADD_DIV_TRACKER }
}

pub fn sub_div_tracker() -> &'static DivTracker {
    unsafe { &SUB_DIV_TRACKER }
}

fn gb_read_mem(regs: &[u32], _stack_pointer: *mut u32) {
    if regs[0] != 0xff04 {
        return;
    }

    let reader = Gen2Reader::crystal();
    let pc = reader.pc_reg();
    const RNG_DIV_READ_1: u16 = 0x2b5 + 1;
    const RNG_DIV_READ_2: u16 = 0x2bd + 1;
    if pc == RNG_DIV_READ_1 {
        let div = reader.div();
        unsafe { ADIV = div };

        unsafe { ADD_DIV_TRACKER.update(div) };
        unsafe { RNG_ADVANCE = RNG_ADVANCE.wrapping_add(1) };
    }
    if pc == RNG_DIV_READ_2 {
        let div = reader.div();
        unsafe { SDIV = div };
        unsafe { SUB_DIV_TRACKER.update(div) };
    }
}

pub fn init_crystal() {
    utils::hook_game_branch!(
        hook_router_addr = 0x2089e4,
        update_cycle_counter = 0x1a8360,
        gb_read_mem = 0x1af17c,
    );
}
