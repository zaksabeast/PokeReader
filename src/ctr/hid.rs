use core::cmp::PartialEq;
use core::ops::{BitAnd, BitOr};
use num_enum::IntoPrimitive;

static mut KEY_ADDR: *mut u32 = core::ptr::null_mut();
static mut CURRENT_KEYS: u32 = 0;
static mut PREVIOUS_KEYS: u32 = 0;

pub unsafe fn set_key_addr(key_addr: *mut u32) {
    KEY_ADDR = key_addr
}

pub fn scan_input() {
    unsafe {
        if KEY_ADDR.is_null() {
            return;
        }

        PREVIOUS_KEYS = CURRENT_KEYS;
        CURRENT_KEYS = KEY_ADDR.read_volatile();
    }
}

fn current_keys() -> u32 {
    unsafe { CURRENT_KEYS }
}

fn previous_keys() -> u32 {
    unsafe { PREVIOUS_KEYS }
}

/// A button that can be pressed by a user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum Button {
    A = 1,
    B = 2,
    Select = 4,
    Start = 8,
    Dright = 16,
    Dleft = 32,
    Dup = 64,
    Ddown = 128,
    R = 256,
    L = 512,
    X = 1024,
    Y = 2048,
}

impl PartialEq<Button> for u32 {
    fn eq(&self, other: &Button) -> bool {
        *self == *other as u32
    }
}

impl PartialEq<u32> for Button {
    fn eq(&self, other: &u32) -> bool {
        *self as u32 == *other
    }
}

impl BitAnd<Button> for u32 {
    type Output = u32;

    fn bitand(self, rhs: Button) -> Self::Output {
        self & (rhs as u32)
    }
}

impl BitOr for Button {
    type Output = u32;

    fn bitor(self, rhs: Self) -> Self::Output {
        (self as u32) | (rhs as u32)
    }
}

impl BitOr<Button> for u32 {
    type Output = u32;

    fn bitor(self, rhs: Button) -> Self::Output {
        self | (rhs as u32)
    }
}

/// Buttons that are pressed now, but weren't pressed last scan.
pub fn just_pressed() -> u32 {
    let prev = previous_keys();
    let curr = current_keys();
    (prev ^ 0xffff_ffff) & curr
}

/// Check if buttons were just pressed.
/// Convenient for one time checks.
///
/// # Examples
/// ```
/// use pnp::{Button, is_just_pressed};
///
/// if is_just_pressed(Button::Dup | Button::Ddown) {
///   // Do something
/// }
/// ```
pub fn is_just_pressed(io_bits: impl Into<u32>) -> bool {
    let io_bits = io_bits.into();
    let just_pressed = just_pressed();
    let curr = current_keys();
    (just_pressed & io_bits) != 0 && io_bits == curr
}

pub fn is_pressing(io_bits: impl Into<u32>) -> bool {
    (current_keys() & io_bits.into()) != 0
}
