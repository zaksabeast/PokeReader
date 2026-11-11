//! Configurable input bindings.
//!
//! PokeReader runs inside the 3DS process, so the only input channel available
//! to it is the console's HID button state. Whether that state comes from a
//! physical console, a keyboard or a gamepad is decided by Citra's input
//! configuration (or by the hardware), not by the plugin.
//!
//! This module provides:
//!
//! - a versioned, persistent configuration ([`InputConfiguration`]),
//! - chord and alternative-chord bindings ([`InputBinding`]),
//! - a pure action dispatcher ([`InputManager`]) that can be tested on the host,
//! - the in-overlay binding editor ([`settings::InputSettingsMenu`]).

mod action;
mod binding;
mod config;
mod manager;
mod repository;
pub mod settings;
#[cfg(test)]
mod tests;

pub use action::InputAction;
pub use manager::InputManager;
pub use repository::{LoadStatus, SaveResult};

use crate::pnp;
use once_cell::unsync::Lazy;
use repository::load;

struct GlobalInput {
    manager: InputManager,
    status: LoadStatus,
    save_path: Option<&'static str>,
    blocks_saving: bool,
}

static mut GLOBAL: Lazy<GlobalInput> = Lazy::new(|| {
    let loaded = load();

    GlobalInput {
        manager: InputManager::new(loaded.config),
        status: loaded.status,
        save_path: loaded.path,
        blocks_saving: loaded.status.blocks_saving(),
    }
});

fn global() -> &'static mut GlobalInput {
    // SAFETY: PokeReader runs single threaded, as documented in the frame
    // modules. The lazy value is initialised on first use.
    unsafe { Lazy::force_mut(&mut GLOBAL) }
}

pub fn input_manager() -> &'static mut InputManager {
    &mut global().manager
}

pub fn load_status() -> LoadStatus {
    global().status
}

/// Saves the current configuration, remembering the path that worked.
pub fn save_configuration() -> SaveResult {
    let state = global();

    if state.blocks_saving {
        return SaveResult::Blocked;
    }

    // Any explicit configuration change replaces the legacy Crystal shortcut.
    state.manager.clear_legacy_crystal_pause();

    let result = repository::save(state.manager.config(), state.save_path);
    if let SaveResult::Saved(path) = result {
        state.save_path = Some(path);
    }

    result
}

/// Enables the Gen 2 `L + R` pause shortcut while the pause binding is the
/// legacy default.
pub fn enable_legacy_crystal_pause() {
    input_manager().enable_legacy_crystal_pause();
}

fn update_manager() -> (u32, u32) {
    let current = pnp::current_buttons();
    let previous = pnp::previous_buttons();
    input_manager().update(current, previous);
    (current, previous)
}

pub fn action_just_pressed(action: InputAction) -> bool {
    let (current, previous) = update_manager();
    input_manager().action_just_pressed(current, previous, action)
}

pub fn should_pause() -> bool {
    let (current, previous) = update_manager();
    input_manager().should_pause(current, previous)
}

pub fn should_step_frame() -> bool {
    let (current, previous) = update_manager();
    input_manager().should_step_frame(current, previous)
}

pub fn should_resume() -> bool {
    let (current, previous) = update_manager();
    input_manager().should_resume(current, previous)
}

/// Called by the C hook to decide whether the game should be paused.
#[no_mangle]
pub extern "C" fn pokereader_should_pause() -> u32 {
    should_pause() as u32
}

/// Called by the C hook when a single frame must be allowed to run.
///
/// This only returns true after the bound input has been released, so the game
/// never observes the button used to advance the frame.
#[no_mangle]
pub extern "C" fn pokereader_should_step_frame() -> u32 {
    should_step_frame() as u32
}

/// Called by the C hook to decide whether the game should be unpaused.
///
/// Like [`pokereader_should_step_frame`], this waits for the bound input to be
/// released before letting the game run.
#[no_mangle]
pub extern "C" fn pokereader_should_resume() -> u32 {
    should_resume() as u32
}
