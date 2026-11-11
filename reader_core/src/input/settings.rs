//! Overlay menu used to edit the input bindings.
//!
//! The overlay is text based, so the whole editor is a small state machine
//! drawn with [`crate::pnp::println`].

use crate::alloc::format;
use crate::alloc::string::String;
use crate::alloc::string::ToString;
use crate::pnp;

use super::{
    InputAction, SaveResult,
    binding::{
        BUTTON_A, BUTTON_B, BUTTON_DDOWN, BUTTON_DRIGHT, BUTTON_DUP, BUTTON_Y, Chord, InputBinding,
        MAX_CHORD_BUTTONS,
    },
    config::{InputConfiguration, InputDevice},
    input_manager, save_configuration,
};

const ROW_DEVICES: usize = 0;
const ROW_PROFILE: usize = 1;
const ROW_ACTION_FIRST: usize = 2;
const ROW_SAFE_PRESET: usize = ROW_ACTION_FIRST + InputAction::COUNT;
const ROW_DEFAULTS: usize = ROW_SAFE_PRESET + 1;
const ROW_CLEAR: usize = ROW_DEFAULTS + 1;
const ROW_TEST: usize = ROW_CLEAR + 1;
const ROW_SAVE: usize = ROW_TEST + 1;
const ROW_COUNT: usize = ROW_SAVE + 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Browse,
    Record,
    Confirm {
        binding: InputBinding,
        conflict: Option<(InputDevice, InputAction)>,
        leaks: bool,
    },
    Test,
}

pub struct InputSettingsMenu {
    cursor: usize,
    edit_device: InputDevice,
    mode: Mode,
    recording_action: Option<InputAction>,
    record_held: u32,
    record_wait_release: bool,
    status: String,
}

impl Default for InputSettingsMenu {
    fn default() -> Self {
        Self {
            cursor: 0,
            edit_device: InputDevice::Keyboard,
            mode: Mode::Browse,
            recording_action: None,
            record_held: 0,
            record_wait_release: false,
            status: String::new(),
        }
    }
}

fn just_pressed(bit: u32) -> bool {
    (pnp::just_pressed_buttons() & bit) != 0
}

fn action_at_row(row: usize) -> Option<InputAction> {
    let index = row.checked_sub(ROW_ACTION_FIRST)?;
    InputAction::ALL.get(index).copied()
}

impl InputSettingsMenu {
    pub fn new() -> Self {
        Self::default()
    }

    fn set_status(&mut self, text: &str) {
        self.status = text.to_string();
    }

    fn save(&mut self) -> SaveResult {
        let result = save_configuration();
        self.status = result.message().to_string();
        result
    }

    fn start_recording(&mut self, action: InputAction) {
        input_manager().set_recording(true);
        self.recording_action = Some(action);
        self.record_held = 0;
        self.record_wait_release = true;
        self.mode = Mode::Record;
        self.status.clear();
    }

    fn cancel_recording(&mut self) {
        input_manager().set_recording(false);
        self.recording_action = None;
        self.record_held = 0;
        self.record_wait_release = false;
        self.mode = Mode::Browse;
        self.set_status("Cancelled");
    }

    fn finish_recording(&mut self, bits: u32) {
        input_manager().set_recording(false);
        self.record_wait_release = false;
        self.record_held = 0;

        if bits.count_ones() > MAX_CHORD_BUTTONS {
            self.mode = Mode::Browse;
            self.set_status(super::binding::BindingParseError::TooManyButtons.description());
            return;
        }

        let binding = InputBinding::single(Chord::new(bits));
        let action = match self.recording_action {
            Some(action) => action,
            None => {
                self.mode = Mode::Browse;
                return;
            }
        };

        let config = *input_manager().config();
        let conflict = config.conflicting_action(action, binding);
        let leaks = InputConfiguration::binding_leaks_to_game(binding);

        if let Some((_, other_action)) = conflict {
            self.status = format!("{} uses it", other_action.label());
            self.mode = Mode::Confirm {
                binding,
                conflict,
                leaks,
            };
            return;
        }

        if leaks {
            self.set_status("Game sees this!");
            self.mode = Mode::Confirm {
                binding,
                conflict: None,
                leaks,
            };
            return;
        }

        self.apply_binding(binding, None);
    }

    fn apply_binding(&mut self, binding: InputBinding, conflict: Option<(InputDevice, InputAction)>) {
        let action = match self.recording_action {
            Some(action) => action,
            None => return,
        };

        if let Some((device, action)) = conflict {
            input_manager().config_mut().clear_binding(device, action);
        }

        input_manager()
            .config_mut()
            .set_binding(self.edit_device, action, binding);
        self.recording_action = None;
        self.mode = Mode::Browse;
        self.save();
    }

    fn clear_action(&mut self, action: InputAction) {
        input_manager()
            .config_mut()
            .clear_binding(self.edit_device, action);
        if self.save().is_ok() {
            self.status = "Cleared".to_string();
        }
    }

    fn activate_row(&mut self) {
        match self.cursor {
            ROW_DEVICES => {
                let active = input_manager().config().active_devices().cycle();
                input_manager().config_mut().set_active_devices(active);
                self.save();
            }
            ROW_PROFILE => {
                self.edit_device = match self.edit_device {
                    InputDevice::Keyboard => InputDevice::Gamepad,
                    InputDevice::Gamepad => InputDevice::Keyboard,
                };
            }
            ROW_SAFE_PRESET => {
                input_manager().set_config(InputConfiguration::conflict_free());
                if self.save().is_ok() {
                    self.status = "Safe preset".to_string();
                }
            }
            ROW_DEFAULTS => {
                input_manager().set_config(InputConfiguration::legacy());
                if self.save().is_ok() {
                    self.status = "Defaults restored".to_string();
                }
            }
            ROW_CLEAR => {
                input_manager().config_mut().clear_profile(self.edit_device);
                if self.save().is_ok() {
                    self.status = "Profile cleared".to_string();
                }
            }
            ROW_TEST => {
                self.mode = Mode::Test;
                self.status.clear();
            }
            ROW_SAVE => {
                self.save();
            }
            row => {
                if let Some(action) = action_at_row(row) {
                    self.start_recording(action);
                }
            }
        }
    }

    fn update_browse(&mut self, is_locked: bool) {
        if is_locked {
            return;
        }

        if just_pressed(BUTTON_DUP) {
            self.cursor = if self.cursor == 0 {
                ROW_COUNT - 1
            } else {
                self.cursor - 1
            };
        } else if just_pressed(BUTTON_DDOWN) {
            self.cursor = (self.cursor + 1) % ROW_COUNT;
        }

        if just_pressed(BUTTON_Y) {
            if let Some(action) = action_at_row(self.cursor) {
                self.clear_action(action);
                return;
            }
        }

        if just_pressed(BUTTON_A) || just_pressed(BUTTON_DRIGHT) {
            self.activate_row();
        }
    }

    fn update_record(&mut self) {
        let current = pnp::current_buttons();

        if self.record_wait_release {
            if current == 0 {
                self.record_wait_release = false;
            }
            return;
        }

        self.record_held |= current;

        if current != 0 || self.record_held == 0 {
            return;
        }

        let bits = self.record_held;
        self.record_held = 0;

        if bits == BUTTON_B {
            self.cancel_recording();
            return;
        }

        self.finish_recording(bits);
    }

    fn update_confirm(&mut self) {
        let (binding, conflict) = match self.mode {
            Mode::Confirm {
                binding, conflict, ..
            } => (binding, conflict),
            _ => return,
        };

        if just_pressed(BUTTON_B) {
            input_manager().set_recording(false);
            self.recording_action = None;
            self.mode = Mode::Browse;
            self.set_status("Cancelled");
            return;
        }

        if just_pressed(BUTTON_A) || just_pressed(BUTTON_DRIGHT) {
            self.apply_binding(binding, conflict);
        }
    }

    pub fn update_and_draw(&mut self, is_locked: bool) {
        let current = pnp::current_buttons();
        let previous = pnp::previous_buttons();
        input_manager().update(current, previous);

        match self.mode {
            Mode::Browse => self.update_browse(is_locked),
            Mode::Record => self.update_record(),
            Mode::Confirm { .. } => self.update_confirm(),
            Mode::Test => {
                if !is_locked && just_pressed(BUTTON_B) {
                    self.mode = Mode::Browse;
                    self.status.clear();
                }
            }
        }

        self.draw();
    }

    fn draw_browse(&mut self) {
        let config = *input_manager().config();
        let active = config.active_devices();

        pnp::println!("Controls");
        pnp::println!("A=Edit Y=Clear");

        for row in 0..ROW_COUNT {
            let cursor = if row == self.cursor { ">" } else { " " };
            match row {
                ROW_DEVICES => pnp::println!("{}Devices {:<6}", cursor, active.compact_label()),
                ROW_PROFILE => pnp::println!("{}Profile {:<6}", cursor, self.edit_device.label()),
                ROW_SAFE_PRESET => pnp::println!("{}Safe preset", cursor),
                ROW_DEFAULTS => pnp::println!("{}Defaults", cursor),
                ROW_CLEAR => pnp::println!("{}Clear profile", cursor),
                ROW_TEST => pnp::println!("{}Test input", cursor),
                ROW_SAVE => pnp::println!("{}Save config", cursor),
                row => {
                    if let Some(action) = action_at_row(row) {
                        let binding = config.binding(self.edit_device, action);
                        pnp::println!("{}{:<7} {}", cursor, action.label(), binding.to_compact_string());
                    }
                }
            }
        }

        pnp::println!("");
        if !self.status.is_empty() {
            pnp::println!("{}", self.status);
        } else {
            pnp::println!("{}", super::load_status().message());
        }
    }

    fn draw_record(&mut self) {
        let label = self.recording_action.map(|action| action.label()).unwrap_or("?");

        pnp::println!("Record {}", label);

        if self.record_wait_release {
            pnp::println!("Release buttons");
            pnp::println!("to begin");
        } else if self.record_held != 0 {
            pnp::println!("Got {}", Chord::new(self.record_held).to_compact_string());
            pnp::println!("Release to set");
        } else {
            pnp::println!("Hold the combo");
            pnp::println!("then release");
        }

        pnp::println!("");
        pnp::println!("[B] alone cancels");
    }

    fn draw_confirm(&mut self) {
        let (binding, conflict, leaks) = match self.mode {
            Mode::Confirm {
                binding,
                conflict,
                leaks,
            } => (binding, conflict, leaks),
            _ => return,
        };

        pnp::println!("Use {}", binding.to_compact_string());
        pnp::println!("");

        if let Some((_, action)) = conflict {
            pnp::println!("{} uses it", action.label());
        }

        if leaks {
            pnp::println!("Game sees this!");
        }

        pnp::println!("");
        pnp::println!("[A] Accept");
        pnp::println!("[B] Cancel");
    }

    fn draw_test(&mut self) {
        let current = input_manager().current_buttons();

        pnp::println!("Test input");
        pnp::println!("Held {}", Chord::new(current).to_compact_string());
        pnp::println!("");

        for action in InputAction::ALL {
            pnp::println!(
                "{} {}",
                action.label(),
                if input_manager().is_action_held(action) {
                    "YES"
                } else {
                    "-"
                }
            );
        }

        pnp::println!("");
        pnp::println!("[B] Back");
    }

    fn draw(&mut self) {
        match self.mode {
            Mode::Browse => self.draw_browse(),
            Mode::Record => self.draw_record(),
            Mode::Confirm { .. } => self.draw_confirm(),
            Mode::Test => self.draw_test(),
        }
    }
}
