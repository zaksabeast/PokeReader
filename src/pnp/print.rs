use crate::ctr;
use core::fmt;

/// Set the max line length.
/// Printed lines with a longer length will be truncated.
pub fn set_print_max_len(max_len: u32) {
    ctr::set_print_max_len(max_len as usize)
}

/// Print to the console screen.
/// Use of the macro is recommended instead of this.
pub fn println_impl(args: fmt::Arguments<'_>, color: u32) {
    ctr::println_impl(args, color)
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
        $crate::pnp::println_impl(core::format_args!(""), 0xffffff)
    };
    (color = $color:expr, $($arg:tt)*) => {{
        $crate::pnp::println_impl(core::format_args!($($arg)*), $color);
    }};
    ($($arg:tt)*) => {{
        $crate::pnp::println_impl(core::format_args!($($arg)*), 0xffffff);
    }};

}

pub use println_impl_macro as println;
