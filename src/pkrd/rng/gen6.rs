#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Gen6Rng {
    advances: u32,
}

impl Gen6Rng {
    pub fn get_advances(&self) -> u32 {
        self.advances
    }
}
