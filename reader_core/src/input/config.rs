use crate::alloc::format;
use crate::alloc::string::String;

use super::{
    action::InputAction,
    binding::{
        BUTTON_A, BUTTON_DUP, BUTTON_SELECT, BUTTON_START, BUTTON_X, BUTTON_Y, BUTTON_ZL, BUTTON_ZR, Chord,
        InputBinding,
    },
};

/// Version of the configuration file format.
pub const CONFIG_VERSION: u16 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputDevice {
    Keyboard,
    Gamepad,
}

impl InputDevice {
    pub const COUNT: usize = 2;
    pub const ALL: [InputDevice; Self::COUNT] = [InputDevice::Keyboard, InputDevice::Gamepad];

    pub const fn index(self) -> usize {
        match self {
            InputDevice::Keyboard => 0,
            InputDevice::Gamepad => 1,
        }
    }

    pub const fn key(self) -> &'static str {
        match self {
            InputDevice::Keyboard => "keyboard",
            InputDevice::Gamepad => "gamepad",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            InputDevice::Keyboard => "Keyboard",
            InputDevice::Gamepad => "Gamepad",
        }
    }

    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            "keyboard" => Some(InputDevice::Keyboard),
            "gamepad" => Some(InputDevice::Gamepad),
            _ => None,
        }
    }
}

/// Which binding profiles are allowed to trigger actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActiveDevices {
    bits: u8,
}

impl ActiveDevices {
    const KEYBOARD_BIT: u8 = 1 << 0;
    const GAMEPAD_BIT: u8 = 1 << 1;

    pub const NONE: ActiveDevices = ActiveDevices { bits: 0 };
    pub const KEYBOARD: ActiveDevices = ActiveDevices {
        bits: Self::KEYBOARD_BIT,
    };
    pub const GAMEPAD: ActiveDevices = ActiveDevices {
        bits: Self::GAMEPAD_BIT,
    };
    pub const BOTH: ActiveDevices = ActiveDevices {
        bits: Self::KEYBOARD_BIT | Self::GAMEPAD_BIT,
    };

    pub const fn contains(self, device: InputDevice) -> bool {
        let bit = match device {
            InputDevice::Keyboard => Self::KEYBOARD_BIT,
            InputDevice::Gamepad => Self::GAMEPAD_BIT,
        };

        (self.bits & bit) != 0 && self.bits != 0
    }

    pub const fn is_disabled(self) -> bool {
        self.bits == 0
    }

    pub fn toggle(self, device: InputDevice) -> Self {
        let bit = match device {
            InputDevice::Keyboard => Self::KEYBOARD_BIT,
            InputDevice::Gamepad => Self::GAMEPAD_BIT,
        };

        Self {
            bits: self.bits ^ bit,
        }
    }

    /// Cycles through both, keyboard, gamepad and disabled.
    pub fn cycle(self) -> Self {
        match self.bits {
            bits if bits == Self::BOTH.bits => Self::KEYBOARD,
            bits if bits == Self::KEYBOARD.bits => Self::GAMEPAD,
            bits if bits == Self::GAMEPAD.bits => Self::NONE,
            _ => Self::BOTH,
        }
    }

    pub fn from_config_str(text: &str) -> Option<Self> {
        let text = text.trim();
        if text.is_empty() {
            return Some(Self::NONE);
        }

        let mut bits = 0;
        for token in text.split(',') {
            match token.trim() {
                "" => {}
                "none" | "disabled" | "off" => return Some(Self::NONE),
                "keyboard" => bits |= Self::KEYBOARD_BIT,
                "gamepad" => bits |= Self::GAMEPAD_BIT,
                "both" | "all" => bits |= Self::KEYBOARD_BIT | Self::GAMEPAD_BIT,
                _ => return None,
            }
        }

        Some(Self { bits })
    }

    pub const fn to_config_string(self) -> &'static str {
        match self.bits {
            0 => "none",
            bits if bits == Self::KEYBOARD_BIT => "keyboard",
            bits if bits == Self::GAMEPAD_BIT => "gamepad",
            _ => "keyboard,gamepad",
        }
    }

    pub const fn compact_label(self) -> &'static str {
        match self.bits {
            0 => "Off",
            bits if bits == Self::KEYBOARD_BIT => "KB",
            bits if bits == Self::GAMEPAD_BIT => "Pad",
            _ => "KB+Pad",
        }
    }
}

/// The bindings of a single device profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProfileBindings {
    bindings: [InputBinding; InputAction::COUNT],
}

impl ProfileBindings {
    pub fn new(bindings: [InputBinding; InputAction::COUNT]) -> Self {
        Self { bindings }
    }

    pub fn binding(&self, action: InputAction) -> InputBinding {
        self.bindings[action.index()]
    }

    fn set_binding(&mut self, action: InputAction, binding: InputBinding) {
        self.bindings[action.index()] = binding;
    }

    fn clear(&mut self) {
        self.bindings = [InputBinding::UNBOUND; InputAction::COUNT];
    }
}

impl Default for ProfileBindings {
    fn default() -> Self {
        Self::new([InputBinding::UNBOUND; InputAction::COUNT])
    }
}

/// Bindings for every action and device, plus the active device selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputConfiguration {
    version: u16,
    keyboard: ProfileBindings,
    gamepad: ProfileBindings,
    active: ActiveDevices,
}

impl InputConfiguration {
    /// The controls used by PokeReader before bindings were configurable.
    ///
    /// This is the default so existing users keep their controls when no
    /// custom configuration exists.
    pub fn legacy() -> Self {
        let pause = InputBinding::single(Chord::new(BUTTON_START | BUTTON_SELECT));
        let step_frame = InputBinding::single(Chord::new(BUTTON_SELECT));
        let resume = InputBinding::pair(Chord::new(BUTTON_START), Chord::new(BUTTON_A));
        let toggle_overlay = InputBinding::single(Chord::new(BUTTON_START | BUTTON_DUP));
        let toggle_lock = InputBinding::single(Chord::new(BUTTON_X | BUTTON_Y));

        Self::with_bindings(pause, step_frame, resume, toggle_overlay, toggle_lock)
    }

    /// A mapping that keeps the gameplay-critical actions isolated.
    ///
    /// Pause, frame step and resume only use the New 3DS `ZL`/`ZR` buttons,
    /// which none of the supported games read. Overlay and lock need a third
    /// button and use `L`/`R`, which the games ignore outside of menus.
    /// Citra (or a New 3DS) is required; on an original 3DS the buttons are
    /// unavailable.
    pub fn conflict_free() -> Self {
        let pause = InputBinding::single(Chord::new(BUTTON_ZL));
        let step_frame = InputBinding::single(Chord::new(BUTTON_ZR));
        let resume = InputBinding::single(Chord::new(BUTTON_ZL | BUTTON_ZR));
        let toggle_overlay =
            InputBinding::single(Chord::new(BUTTON_ZL | BUTTON_ZR | super::binding::BUTTON_L));
        let toggle_lock = InputBinding::single(Chord::new(BUTTON_ZL | BUTTON_ZR | super::binding::BUTTON_R));

        Self::with_bindings(pause, step_frame, resume, toggle_overlay, toggle_lock)
    }

    fn with_bindings(
        pause: InputBinding,
        step_frame: InputBinding,
        resume: InputBinding,
        toggle_overlay: InputBinding,
        toggle_lock: InputBinding,
    ) -> Self {
        let mut bindings = [InputBinding::UNBOUND; InputAction::COUNT];
        bindings[InputAction::Pause.index()] = pause;
        bindings[InputAction::StepFrame.index()] = step_frame;
        bindings[InputAction::Resume.index()] = resume;
        bindings[InputAction::ToggleOverlay.index()] = toggle_overlay;
        bindings[InputAction::ToggleLock.index()] = toggle_lock;
        let profile = ProfileBindings::new(bindings);

        Self {
            version: CONFIG_VERSION,
            keyboard: profile,
            gamepad: profile,
            active: ActiveDevices::BOTH,
        }
    }

    pub const fn active_devices(&self) -> ActiveDevices {
        self.active
    }

    pub fn set_active_devices(&mut self, active: ActiveDevices) {
        self.active = active;
    }

    pub fn is_device_active(&self, device: InputDevice) -> bool {
        self.active.contains(device)
    }

    pub fn profile(&self, device: InputDevice) -> &ProfileBindings {
        match device {
            InputDevice::Keyboard => &self.keyboard,
            InputDevice::Gamepad => &self.gamepad,
        }
    }

    fn profile_mut(&mut self, device: InputDevice) -> &mut ProfileBindings {
        match device {
            InputDevice::Keyboard => &mut self.keyboard,
            InputDevice::Gamepad => &mut self.gamepad,
        }
    }

    pub fn binding(&self, device: InputDevice, action: InputAction) -> InputBinding {
        self.profile(device).binding(action)
    }

    pub fn set_binding(&mut self, device: InputDevice, action: InputAction, binding: InputBinding) {
        self.profile_mut(device).set_binding(action, binding);
    }

    pub fn clear_binding(&mut self, device: InputDevice, action: InputAction) {
        self.set_binding(device, action, InputBinding::UNBOUND);
    }

    pub fn clear_profile(&mut self, device: InputDevice) {
        self.profile_mut(device).clear();
    }

    pub fn clear_all_bindings(&mut self) {
        self.keyboard.clear();
        self.gamepad.clear();
    }

    /// Finds another action that already uses the same chord.
    ///
    /// A more specific chord (for example `ZL+ZR` for resume and `ZL` for
    /// pause) is intentional and is not reported as a conflict.
    pub fn conflicting_action(
        &self,
        action: InputAction,
        binding: InputBinding,
    ) -> Option<(InputDevice, InputAction)> {
        for device in InputDevice::ALL {
            for other_action in InputAction::ALL {
                if other_action == action {
                    continue;
                }

                let other = self.binding(device, other_action);
                for chord in binding.chords() {
                    if other.chords().iter().any(|other_chord| other_chord == chord) {
                        return Some((device, other_action));
                    }
                }
            }
        }

        None
    }

    /// Returns true when any button of the binding is also used by the games.
    pub fn binding_leaks_to_game(binding: InputBinding) -> bool {
        let mut remaining = binding.buttons();

        while remaining != 0 {
            let bit = 1u32 << remaining.trailing_zeros();
            if InputAction::is_gameplay_button(bit) {
                return true;
            }
            remaining &= !bit;
        }

        false
    }

    pub fn to_config_string(&self) -> String {
        let mut text = String::new();
        text.push_str("# PokeReader input configuration\n");
        text.push_str("# See INPUT_CONFIGURATION.md for the file format.\n");
        text.push_str(&format!("version={}\n", CONFIG_VERSION));
        text.push_str(&format!("active={}\n", self.active.to_config_string()));
        text.push('\n');

        for device in InputDevice::ALL {
            text.push_str(&format!("# {}\n", device.key()));
            for action in InputAction::ALL {
                text.push_str(&format!(
                    "{}.{}={}\n",
                    device.key(),
                    action.key(),
                    self.binding(device, action).to_config_string()
                ));
            }
            text.push('\n');
        }

        text
    }
}

impl Default for InputConfiguration {
    fn default() -> Self {
        Self::legacy()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParsedConfiguration {
    pub config: InputConfiguration,
    pub warnings: u16,
    /// True when the file had no version, or an older version.
    pub migrated: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigParseError {
    UnsupportedVersion(u16),
}

/// Parses the configuration file.
///
/// Unknown keys are ignored so that files written by newer versions keep
/// working. Invalid values fall back to the default for that key and are
/// reported through [`ParsedConfiguration::warnings`].
pub fn parse_config(text: &str) -> Result<ParsedConfiguration, ConfigParseError> {
    let mut config = InputConfiguration::legacy();
    let mut warnings = 0;
    let mut version = 0;
    let mut saw_version = false;

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let (key, value) = match line.split_once('=') {
            Some(parts) => parts,
            None => {
                warnings += 1;
                continue;
            }
        };

        let key = key.trim();
        let value = value.trim();

        match key {
            "version" => match value.parse::<u16>() {
                Ok(parsed) => {
                    version = parsed;
                    saw_version = true;
                }
                Err(_) => warnings += 1,
            },
            "active" => match ActiveDevices::from_config_str(value) {
                Some(active) => config.set_active_devices(active),
                None => warnings += 1,
            },
            _ => {
                let mut parts = key.split('.');
                let device = parts.next().and_then(InputDevice::from_key);
                let action = parts.next().and_then(InputAction::from_key);
                let has_extra = parts.next().is_some();

                match (device, action, has_extra) {
                    (Some(device), Some(action), false) => match InputBinding::parse(value) {
                        Ok(binding) => config.set_binding(device, action, binding),
                        Err(_) => warnings += 1,
                    },
                    _ => {
                        // Unknown key from a newer version: ignore it.
                    }
                }
            }
        }
    }

    if version > CONFIG_VERSION {
        return Err(ConfigParseError::UnsupportedVersion(version));
    }

    config.version = CONFIG_VERSION;

    Ok(ParsedConfiguration {
        config,
        warnings,
        migrated: !saw_version || version < CONFIG_VERSION,
    })
}
