use super::bindings;

/// Set the max line length.
/// Printed lines with a longer length will be truncated.
pub fn set_print_max_len(max_len: u32) {
    unsafe { bindings::host_set_print_max_len(max_len) }
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
