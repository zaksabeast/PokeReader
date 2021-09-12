use core::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Stats {
    pub hp: u32,
    pub atk: u32,
    pub def: u32,
    pub spa: u32,
    pub spd: u32,
    pub spe: u32,
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}/{}/{}/{}/{}/{}",
            self.hp, self.atk, self.def, self.spa, self.spd, self.spe
        )
    }
}
