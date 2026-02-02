use super::bindings;

/// Returns the running title id.
pub fn title_id() -> u64 {
    unsafe { bindings::host_get_game_title_id() }
}

pub fn update_version() -> u16 {
    unsafe { bindings::get_remaster_version() }
}

pub fn is_citra() -> bool {
    unsafe { bindings::is_citra() }
}
