use super::{
    action::InputAction,
    binding::InputBinding,
    config::{InputConfiguration, InputDevice},
};

/// Tracks the state of the buttons and turns them into PokeReader actions.
///
/// This type is pure: it receives the raw button state as arguments. That
/// keeps it testable on the host and keeps all platform access in
/// [`crate::pnp`].
pub struct InputManager {
    config: InputConfiguration,
    events: u32,
    step_pending: bool,
    resume_pending: bool,
    recording: bool,
    extra_pause: Option<super::binding::Chord>,
    last_current: u32,
    last_previous: u32,
    has_state: bool,
}

impl InputManager {
    pub fn new(config: InputConfiguration) -> Self {
        Self {
            config,
            events: 0,
            step_pending: false,
            resume_pending: false,
            recording: false,
            extra_pause: None,
            last_current: 0,
            last_previous: 0,
            has_state: false,
        }
    }

    /// Keeps the Crystal (Gen 2) `L + R` pause shortcut working without
    /// changing the configuration of every other title.
    ///
    /// The shortcut is only enabled while the pause binding is still the
    /// legacy default, so a custom binding always wins.
    pub fn enable_legacy_crystal_pause(&mut self) {
        let legacy = InputBinding::single(super::binding::Chord::new(
            super::binding::BUTTON_START | super::binding::BUTTON_SELECT,
        ));

        let is_legacy = InputDevice::ALL
            .iter()
            .all(|device| self.config.binding(*device, InputAction::Pause) == legacy);

        if is_legacy {
            self.extra_pause = Some(super::binding::Chord::new(
                super::binding::BUTTON_L | super::binding::BUTTON_R,
            ));
        }
    }

    /// Removes the Crystal shortcut after the configuration is edited.
    pub fn clear_legacy_crystal_pause(&mut self) {
        self.extra_pause = None;
    }

    pub fn config(&self) -> &InputConfiguration {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut InputConfiguration {
        &mut self.config
    }

    pub fn set_config(&mut self, config: InputConfiguration) {
        self.config = config;
        self.events = 0;
        self.step_pending = false;
        self.resume_pending = false;
    }

    pub fn set_recording(&mut self, recording: bool) {
        self.recording = recording;
    }

    pub fn current_buttons(&self) -> u32 {
        self.last_current
    }

    /// Updates the internal state.
    ///
    /// `current` and `previous` are the raw button masks read from the console
    /// on this poll and the previous one.
    pub fn update(&mut self, current: u32, previous: u32) {
        if self.has_state && current == self.last_current && previous == self.last_previous {
            return;
        }

        self.has_state = true;
        self.last_current = current;
        self.last_previous = previous;

        let just_pressed = current & !previous;
        self.events = self.compute_events(current, just_pressed);

        if self.events & action_bit(InputAction::StepFrame) != 0 {
            self.step_pending = true;
        }

        if self.events & action_bit(InputAction::Resume) != 0 {
            self.resume_pending = true;
        }
    }

    fn compute_events(&self, current: u32, just_pressed: u32) -> u32 {
        if self.recording || self.config.active_devices().is_disabled() {
            return 0;
        }

        let mut best: Option<(u32, InputAction)> = None;

        for device in InputDevice::ALL {
            if !self.config.is_device_active(device) {
                continue;
            }

            for action in InputAction::ALL {
                let binding = self.config.binding(device, action);
                let matched = binding.matched_len(just_pressed, current);
                if matched == 0 {
                    continue;
                }

                best = Self::better_match(best, matched, action);
            }
        }

        if let Some(chord) = self.extra_pause {
            if (just_pressed & chord.bits()) != 0 && chord.is_held(current) {
                best = Self::better_match(best, chord.len(), InputAction::Pause);
            }
        }

        match best {
            Some((_, action)) => action_bit(action),
            None => 0,
        }
    }

    fn better_match(
        best: Option<(u32, InputAction)>,
        matched: u32,
        action: InputAction,
    ) -> Option<(u32, InputAction)> {
        let is_better = match best {
            None => true,
            Some((best_len, best_action)) => {
                matched > best_len || (matched == best_len && action.index() < best_action.index())
            }
        };

        if is_better { Some((matched, action)) } else { best }
    }

    pub fn action_just_pressed(&mut self, current: u32, previous: u32, action: InputAction) -> bool {
        self.update(current, previous);
        (self.events & action_bit(action)) != 0
    }

    pub fn should_pause(&mut self, current: u32, previous: u32) -> bool {
        self.action_just_pressed(current, previous, InputAction::Pause)
    }

    /// Returns true once the frame-step input has been released.
    ///
    /// Waiting for the release guarantees that the game never sees the button
    /// that advanced the frame.
    pub fn should_step_frame(&mut self, current: u32, previous: u32) -> bool {
        self.update(current, previous);

        if !self.step_pending {
            return false;
        }

        if current & self.gate_buttons(InputAction::StepFrame) != 0 {
            return false;
        }

        self.step_pending = false;
        true
    }

    /// Returns true once the resume input has been released.
    pub fn should_resume(&mut self, current: u32, previous: u32) -> bool {
        self.update(current, previous);

        if !self.resume_pending {
            return false;
        }

        if current & self.gate_buttons(InputAction::Resume) != 0 {
            return false;
        }

        self.resume_pending = false;
        true
    }

    /// Every button that may trigger `action` in any active profile.
    pub fn gate_buttons(&self, action: InputAction) -> u32 {
        let mut bits = 0;

        for device in InputDevice::ALL {
            if self.config.is_device_active(device) {
                bits |= self.config.binding(device, action).buttons();
            }
        }

        if action == InputAction::Pause {
            if let Some(chord) = self.extra_pause {
                bits |= chord.bits();
            }
        }

        bits
    }

    pub fn binding_for(&self, action: InputAction) -> InputBinding {
        for device in InputDevice::ALL {
            if self.config.is_device_active(device) {
                let binding = self.config.binding(device, action);
                if !binding.is_unbound() {
                    return binding;
                }
            }
        }

        InputBinding::UNBOUND
    }

    /// True while any chord of the action is held, without requiring an edge.
    pub fn is_action_held(&self, action: InputAction) -> bool {
        let held = self.last_current;

        for device in InputDevice::ALL {
            if !self.config.is_device_active(device) {
                continue;
            }

            let binding = self.config.binding(device, action);
            for chord in binding.chords() {
                if chord.is_held(held) {
                    return true;
                }
            }
        }

        if action == InputAction::Pause {
            if let Some(chord) = self.extra_pause {
                return chord.is_held(held);
            }
        }

        false
    }
}

pub const fn action_bit(action: InputAction) -> u32 {
    1 << action.index()
}
