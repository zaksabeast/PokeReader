use super::tinymt;
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Gen6Rng {
    init_seed: usize,
    init_tinymt_state: [u32; 4],
    last_mt_state_index: usize,
    mt_advances: u32,
    tinymt_rng: tinymt::TinyMT,
    tinymt_advances: u32,
}

impl Gen6Rng {
    pub fn get_mt_advances(&self) -> u32 {
        self.mt_advances
    }

    pub fn get_tinymt_advances(&self) -> u32 {
        self.tinymt_advances
    }

    pub fn get_initial_tinymt_state(&self) -> [u32; 4] {
        self.init_tinymt_state
    }

    pub fn get_tinymt_state(&self) -> [u32; 4] {
        self.tinymt_rng.get_state()
    }

    pub fn update(&mut self, mt_state_index: usize, init_seed: usize, tinymt_state: [u32; 4]) {
        if self.init_seed != init_seed && init_seed != 0 {
            self.last_mt_state_index = 0;
            self.mt_advances = 0;
            self.tinymt_advances = 0;
            self.init_seed = init_seed;
            self.init_tinymt_state = tinymt_state;
            self.tinymt_rng = tinymt::TinyMT::new(tinymt_state)
        }

        if mt_state_index > self.last_mt_state_index {
            self.mt_advances += (mt_state_index - self.last_mt_state_index) as u32;
        } else if mt_state_index < self.last_mt_state_index {
            self.mt_advances += (624 - self.last_mt_state_index + mt_state_index) as u32;
        }

        while tinymt_state != self.get_tinymt_state() && tinymt_state != [0, 0, 0, 0] {
            self.tinymt_rng.next_state();
            self.tinymt_advances += 1;
        }
        self.last_mt_state_index = mt_state_index;
    }
}
