#[macro_export]
macro_rules! func_ptr {
    ($func:expr) => {
        $func as *const () as u32
    };
}
