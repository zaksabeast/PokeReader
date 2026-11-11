use crate::alloc::string::String;
use crate::alloc::string::ToString;

pub const BUTTON_A: u32 = 1 << 0;
pub const BUTTON_B: u32 = 1 << 1;
pub const BUTTON_SELECT: u32 = 1 << 2;
pub const BUTTON_START: u32 = 1 << 3;
pub const BUTTON_DRIGHT: u32 = 1 << 4;
pub const BUTTON_DLEFT: u32 = 1 << 5;
pub const BUTTON_DUP: u32 = 1 << 6;
pub const BUTTON_DDOWN: u32 = 1 << 7;
pub const BUTTON_R: u32 = 1 << 8;
pub const BUTTON_L: u32 = 1 << 9;
pub const BUTTON_X: u32 = 1 << 10;
pub const BUTTON_Y: u32 = 1 << 11;
/// Only available on a New 3DS (or when Citra emulates one).
pub const BUTTON_ZL: u32 = 1 << 14;
/// Only available on a New 3DS (or when Citra emulates one).
pub const BUTTON_ZR: u32 = 1 << 15;

/// Every button PokeReader knows how to bind.
pub const KNOWN_BUTTONS: u32 = BUTTON_A
    | BUTTON_B
    | BUTTON_SELECT
    | BUTTON_START
    | BUTTON_DRIGHT
    | BUTTON_DLEFT
    | BUTTON_DUP
    | BUTTON_DDOWN
    | BUTTON_R
    | BUTTON_L
    | BUTTON_X
    | BUTTON_Y
    | BUTTON_ZL
    | BUTTON_ZR;

/// Order used when printing a chord.
///
/// Keeping this order stable makes configuration files and overlay text
/// predictable (`Start+Select` instead of `Select+Start`).
pub const BUTTON_DISPLAY_ORDER: [u32; 14] = [
    BUTTON_A,
    BUTTON_B,
    BUTTON_X,
    BUTTON_Y,
    BUTTON_START,
    BUTTON_SELECT,
    BUTTON_L,
    BUTTON_R,
    BUTTON_ZL,
    BUTTON_ZR,
    BUTTON_DUP,
    BUTTON_DDOWN,
    BUTTON_DLEFT,
    BUTTON_DRIGHT,
];

/// Maximum number of buttons that can be held at once for a single chord.
pub const MAX_CHORD_BUTTONS: u32 = 4;

/// Maximum number of alternative chords for a single action.
pub const MAX_ALTERNATIVES: usize = 2;

/// A single combination of buttons pressed at the same time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Chord {
    bits: u32,
}

impl Chord {
    pub const EMPTY: Chord = Chord { bits: 0 };

    pub const fn new(bits: u32) -> Self {
        Self {
            bits: bits & KNOWN_BUTTONS,
        }
    }

    pub const fn bits(self) -> u32 {
        self.bits
    }

    pub const fn is_empty(self) -> bool {
        self.bits == 0
    }

    pub const fn len(self) -> u32 {
        self.bits.count_ones()
    }

    /// Returns true when every button of the chord is currently held.
    pub const fn is_held(self, held: u32) -> bool {
        !self.is_empty() && (held & self.bits) == self.bits
    }

    pub fn buttons(self) -> impl Iterator<Item = u32> {
        let bits = self.bits;
        BUTTON_DISPLAY_ORDER
            .iter()
            .copied()
            .filter(move |bit| (bits & bit) != 0)
    }

    fn append_names(self, text: &mut String, separator: &str) {
        let mut first = true;
        for bit in self.buttons() {
            if !first {
                text.push_str(separator);
            }
            text.push_str(button_name(bit));
            first = false;
        }
    }

    /// Human readable representation, e.g. `Start + Select`.
    pub fn to_display_string(self) -> String {
        let mut text = String::new();
        self.append_names(&mut text, " + ");
        text
    }

    /// Compact representation used by the overlay, e.g. `Start+Select`.
    pub fn to_compact_string(self) -> String {
        let mut text = String::new();
        self.append_names(&mut text, "+");
        text
    }
}

pub const fn button_name(bit: u32) -> &'static str {
    match bit {
        BUTTON_A => "A",
        BUTTON_B => "B",
        BUTTON_SELECT => "Select",
        BUTTON_START => "Start",
        BUTTON_DRIGHT => "Right",
        BUTTON_DLEFT => "Left",
        BUTTON_DUP => "Up",
        BUTTON_DDOWN => "Down",
        BUTTON_R => "R",
        BUTTON_L => "L",
        BUTTON_X => "X",
        BUTTON_Y => "Y",
        BUTTON_ZL => "ZL",
        BUTTON_ZR => "ZR",
        _ => "?",
    }
}

pub fn button_bit(name: &str) -> Option<u32> {
    let name = name.trim();
    let candidates = [
        ("a", BUTTON_A),
        ("b", BUTTON_B),
        ("select", BUTTON_SELECT),
        ("start", BUTTON_START),
        ("right", BUTTON_DRIGHT),
        ("dright", BUTTON_DRIGHT),
        ("d-right", BUTTON_DRIGHT),
        ("left", BUTTON_DLEFT),
        ("dleft", BUTTON_DLEFT),
        ("d-left", BUTTON_DLEFT),
        ("up", BUTTON_DUP),
        ("dup", BUTTON_DUP),
        ("d-up", BUTTON_DUP),
        ("down", BUTTON_DDOWN),
        ("ddown", BUTTON_DDOWN),
        ("d-down", BUTTON_DDOWN),
        ("r", BUTTON_R),
        ("l", BUTTON_L),
        ("x", BUTTON_X),
        ("y", BUTTON_Y),
        ("zl", BUTTON_ZL),
        ("zr", BUTTON_ZR),
    ];

    candidates
        .iter()
        .find(|(candidate, _)| name.eq_ignore_ascii_case(candidate))
        .map(|(_, bit)| *bit)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingParseError {
    UnknownButton,
    TooManyButtons,
    TooManyAlternatives,
}

impl BindingParseError {
    pub fn description(self) -> &'static str {
        match self {
            BindingParseError::UnknownButton => "Unknown button",
            BindingParseError::TooManyButtons => "Too many buttons",
            BindingParseError::TooManyAlternatives => "Too many bindings",
        }
    }
}

/// The binding of a single action.
///
/// A binding is made of up to [`MAX_ALTERNATIVES`] chords. Any of them can
/// trigger the action, which mirrors the previous "Start or A unpauses"
/// behaviour while still allowing button combinations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct InputBinding {
    chords: [Chord; MAX_ALTERNATIVES],
    count: usize,
}

impl InputBinding {
    pub const UNBOUND: InputBinding = InputBinding {
        chords: [Chord::EMPTY; MAX_ALTERNATIVES],
        count: 0,
    };

    pub fn single(chord: Chord) -> Self {
        if chord.is_empty() {
            return Self::UNBOUND;
        }

        let mut binding = Self::UNBOUND;
        binding.chords[0] = chord;
        binding.count = 1;
        binding
    }

    /// Builds a binding made of a primary chord and an alternative, ignoring
    /// empty chords. Used for defaults such as "Start or A".
    pub fn pair(primary: Chord, alternative: Chord) -> Self {
        let mut binding = Self::UNBOUND;

        if !primary.is_empty() {
            binding.chords[binding.count] = primary;
            binding.count += 1;
        }

        if !alternative.is_empty() && binding.count < MAX_ALTERNATIVES {
            binding.chords[binding.count] = alternative;
            binding.count += 1;
        }

        binding
    }

    pub fn from_chords(chords: &[Chord]) -> Result<Self, BindingParseError> {
        let mut binding = Self::UNBOUND;
        for chord in chords {
            if chord.is_empty() {
                continue;
            }

            if binding.count == MAX_ALTERNATIVES {
                return Err(BindingParseError::TooManyAlternatives);
            }

            binding.chords[binding.count] = *chord;
            binding.count += 1;
        }

        Ok(binding)
    }

    /// Adds an alternative chord, keeping the binding unchanged if there is no
    /// room left.
    pub fn with_alternative(mut self, chord: Chord) -> Result<Self, BindingParseError> {
        if chord.is_empty() || self.count == MAX_ALTERNATIVES {
            return Err(BindingParseError::TooManyAlternatives);
        }

        self.chords[self.count] = chord;
        self.count += 1;
        Ok(self)
    }

    pub fn is_unbound(self) -> bool {
        self.count == 0
    }

    pub fn chords(&self) -> &[Chord] {
        &self.chords[..self.count]
    }

    pub fn buttons(self) -> u32 {
        let mut bits = 0;
        for chord in self.chords() {
            bits |= chord.bits();
        }
        bits
    }

    /// Returns true when one of the chords is fully held and at least one of its
    /// buttons was pressed on this frame.
    pub fn matches(self, just_pressed: u32, held: u32) -> bool {
        self.chords()
            .iter()
            .any(|chord| (just_pressed & chord.bits()) != 0 && chord.is_held(held))
    }

    /// Number of buttons of the largest chord that matches.
    pub fn matched_len(self, just_pressed: u32, held: u32) -> u32 {
        self.chords()
            .iter()
            .filter(|chord| (just_pressed & chord.bits()) != 0 && chord.is_held(held))
            .map(|chord| chord.len())
            .max()
            .unwrap_or(0)
    }

    /// Compact representation used by the overlay, e.g. `Start+Select / A`.
    pub fn to_compact_string(self) -> String {
        if self.is_unbound() {
            return "None".to_string();
        }

        let mut text = String::new();
        for (index, chord) in self.chords().iter().enumerate() {
            if index != 0 {
                text.push_str("/");
            }
            text.push_str(&chord.to_compact_string());
        }
        text
    }

    /// Human readable representation, e.g. `Start + Select / A`.
    pub fn to_display_string(self) -> String {
        if self.is_unbound() {
            return "None".to_string();
        }

        let mut text = String::new();
        for (index, chord) in self.chords().iter().enumerate() {
            if index != 0 {
                text.push_str(" / ");
            }
            text.push_str(&chord.to_display_string());
        }
        text
    }

    /// Serialized representation used by the configuration file.
    pub fn to_config_string(self) -> String {
        if self.is_unbound() {
            return "-".to_string();
        }

        let mut text = String::new();
        for (index, chord) in self.chords().iter().enumerate() {
            if index != 0 {
                text.push('|');
            }
            text.push_str(&chord.to_compact_string());
        }
        text
    }

    pub fn parse(text: &str) -> Result<Self, BindingParseError> {
        let text = text.trim();
        if text.is_empty() || text == "-" {
            return Ok(Self::UNBOUND);
        }

        let mut chords = [Chord::EMPTY; MAX_ALTERNATIVES];
        let mut count = 0;

        for alternative in text.split('|') {
            let mut bits = 0;
            let mut buttons = 0;

            for name in alternative.split('+') {
                let name = name.trim();
                if name.is_empty() {
                    continue;
                }

                let bit = button_bit(name).ok_or(BindingParseError::UnknownButton)?;
                bits |= bit;
                buttons += 1;
            }

            if buttons == 0 {
                continue;
            }

            if buttons > MAX_CHORD_BUTTONS {
                return Err(BindingParseError::TooManyButtons);
            }

            if count == MAX_ALTERNATIVES {
                return Err(BindingParseError::TooManyAlternatives);
            }

            chords[count] = Chord::new(bits);
            count += 1;
        }

        Ok(Self { chords, count })
    }
}
