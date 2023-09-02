use super::bindings;

/// Resets the print settings.
pub fn reset_print() {
    unsafe { bindings::host_reset_print() }
}

/// Set the text and background colors.
pub fn set_print_colors(text_color: u32, background_color: u32) {
    unsafe { bindings::host_set_print_colors(text_color, background_color) }
}

/// Set the max line length.
/// Printed lines with a longer length will be truncated.
pub fn set_print_max_len(max_len: u32) {
    unsafe { bindings::host_set_print_max_len(max_len) }
}

/// Sets the printer's current X position.
pub fn set_print_x(x: u32) {
    unsafe { bindings::host_set_print_x(x) }
}

/// Sets the printer's current Y position.
pub fn set_print_y(y: u32) {
    unsafe { bindings::host_set_print_y(y) }
}

/// Print to the console screen.
/// Use of the macro is recommended instead of this.
pub fn println_impl(text: &str) {
    unsafe { bindings::host_print(text.as_ptr() as u32, text.len() as u32) }
}

/// Prints to the console screen.
///
/// # Examples
/// ```
/// pnp::println!("Test: {:x}", 0xaabbccddu32);
/// ```
#[macro_export]
macro_rules! println_impl_macro {
    () => {
        $crate::pnp::println_impl("")
    };
    ($($arg:tt)*) => {{
        $crate::pnp::println_impl(&alloc::format!($($arg)*));
    }};
}

pub use println_impl_macro as println;
