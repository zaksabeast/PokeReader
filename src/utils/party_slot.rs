use super::CircularCounter;
use core::fmt;

pub type PartySlot = CircularCounter<1, 6>;

impl fmt::Display for PartySlot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
