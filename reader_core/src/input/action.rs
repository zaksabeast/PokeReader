/// An action that can be triggered by the user.
///
/// PokeReader runs inside the 3DS process, so the only input it can observe is
/// the 3DS HID button state. Whether those buttons come from a keyboard, a
/// gamepad or the console itself is decided outside of the plugin (by Citra's
/// input configuration or by the physical console).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputAction {
    Pause,
    StepFrame,
    Resume,
    ToggleOverlay,
    ToggleLock,
}

impl InputAction {
    pub const COUNT: usize = 5;

    pub const ALL: [InputAction; Self::COUNT] = [
        InputAction::Pause,
        InputAction::StepFrame,
        InputAction::Resume,
        InputAction::ToggleOverlay,
        InputAction::ToggleLock,
    ];

    pub const fn index(self) -> usize {
        match self {
            InputAction::Pause => 0,
            InputAction::StepFrame => 1,
            InputAction::Resume => 2,
            InputAction::ToggleOverlay => 3,
            InputAction::ToggleLock => 4,
        }
    }

    /// Stable identifier used by the configuration file.
    pub const fn key(self) -> &'static str {
        match self {
            InputAction::Pause => "pause",
            InputAction::StepFrame => "step_frame",
            InputAction::Resume => "resume",
            InputAction::ToggleOverlay => "toggle_overlay",
            InputAction::ToggleLock => "toggle_lock",
        }
    }

    /// Short label used by the overlay menu.
    pub const fn label(self) -> &'static str {
        match self {
            InputAction::Pause => "Pause",
            InputAction::StepFrame => "Step",
            InputAction::Resume => "Resume",
            InputAction::ToggleOverlay => "Overlay",
            InputAction::ToggleLock => "Lock",
        }
    }

    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            "pause" => Some(InputAction::Pause),
            "step_frame" => Some(InputAction::StepFrame),
            "resume" => Some(InputAction::Resume),
            "toggle_overlay" => Some(InputAction::ToggleOverlay),
            "toggle_lock" => Some(InputAction::ToggleLock),
            _ => None,
        }
    }

    /// Buttons that are usually consumed by the games PokeReader supports.
    ///
    /// Used to warn the user when a binding is likely to leak into gameplay.
    /// This is only a warning: the user is always allowed to keep the binding.
    pub fn is_gameplay_button(bit: u32) -> bool {
        const GAMEPLAY_BUTTONS: u32 = super::binding::BUTTON_A
            | super::binding::BUTTON_B
            | super::binding::BUTTON_SELECT
            | super::binding::BUTTON_START
            | super::binding::BUTTON_DRIGHT
            | super::binding::BUTTON_DLEFT
            | super::binding::BUTTON_DUP
            | super::binding::BUTTON_DDOWN
            | super::binding::BUTTON_R
            | super::binding::BUTTON_L
            | super::binding::BUTTON_X
            | super::binding::BUTTON_Y;

        (bit & GAMEPLAY_BUTTONS) != 0
    }
}
