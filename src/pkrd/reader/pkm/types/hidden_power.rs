use core::fmt;
use num_enum::FromPrimitive;

#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum HiddenPower {
    Fighting = 0,
    Flying = 1,
    Poison = 2,
    Ground = 3,
    Rock = 4,
    Bug = 5,
    Ghost = 6,
    Steel = 7,
    Fire = 8,
    Water = 9,
    Grass = 10,
    Electric = 11,
    Psychic = 12,
    Ice = 13,
    Dragon = 14,
    Dark = 15,
    #[num_enum(default)]
    Invalid = 16,
}

impl fmt::Display for HiddenPower {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
