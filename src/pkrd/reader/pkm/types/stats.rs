use core::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Stats {
    pub hp: u8,
    pub atk: u8,
    pub def: u8,
    pub spa: u8,
    pub spd: u8,
    pub spe: u8,
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02}/{:02}/{:02}/{:02}/{:02}/{:02}",
            self.hp, self.atk, self.def, self.spa, self.spd, self.spe
        )
    }
}
