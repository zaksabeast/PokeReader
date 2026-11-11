use super::bindings;
use core::{
    cmp::PartialEq,
    ops::{BitAnd, BitOr},
};
use num_enum::IntoPrimitive;

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
    /// Only available on a New 3DS (or when Citra emulates one).
    ZL = 0x4000,
    /// Only available on a New 3DS (or when Citra emulates one).
    ZR = 0x8000,
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
    let is_pressed = unsafe { bindings::host_is_just_pressed(io_bits.into()) };
    is_pressed != 0
}

pub fn is_pressing(io_bits: impl Into<u32>) -> bool {
    let current_keys = unsafe { bindings::get_current_keys() };
    (current_keys & io_bits.into()) != 0
}

/// The raw button state read on the last frame.
pub fn current_buttons() -> u32 {
    unsafe { bindings::get_current_keys() }
}

/// The raw button state read on the previous frame.
pub fn previous_buttons() -> u32 {
    unsafe { bindings::get_previous_keys() }
}

/// Buttons that were pressed between the two last frames.
pub fn just_pressed_buttons() -> u32 {
    current_buttons() & !previous_buttons()
}
