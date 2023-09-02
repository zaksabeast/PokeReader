use super::bindings;

/// Returns the running title id.
pub fn title_id() -> u64 {
    unsafe { bindings::host_get_game_title_id() }
}

/// Returns whether the module is running with mode3 features.
pub fn is_mode_3() -> bool {
    unsafe { bindings::host_get_is_mode3() != 0 }
}
