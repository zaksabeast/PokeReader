use super::tinymt;
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Gen6Rng {
    init_seed: usize,
    init_tinymt_state: [u32; 4],
    last_mt_state_index: usize,
    last_tinymt_state: [u32; 4],
    advances: u32,
    index: u32,
}

impl Gen6Rng {
    pub fn get_advances(&self) -> u32 {
        self.advances
    }

    pub fn get_index(&self) -> u32 {
        self.index
    }

    pub fn get_initial_tinymt_state(&self) -> [u32;4] {
        self.init_tinymt_state
    }

    pub fn get_tinymt_state(&self) -> [u32;4] {
        self.last_tinymt_state
    }

    pub fn update(&mut self, mt_state_index : usize, init_seed : usize, tinymt_state : [u32;4]) {
        if self.init_seed != init_seed && init_seed != 0 {
            self.last_mt_state_index = 0;
            self.advances = 0;
            self.index = 0;
            self.init_seed = init_seed;
            self.init_tinymt_state = tinymt_state;
            self.last_tinymt_state = tinymt_state;
        }

        if mt_state_index > self.last_mt_state_index {
            self.advances += (mt_state_index - self.last_mt_state_index) as u32;
        }
        else if mt_state_index < self.last_mt_state_index {
            self.advances += (624 - self.last_mt_state_index + mt_state_index) as u32;
        }

        while tinymt_state != self.last_tinymt_state && tinymt_state != [0,0,0,0] {
            tinymt::TinyMT::next_state(&mut self.last_tinymt_state);
            self.index += 1;
        }
        self.last_mt_state_index = mt_state_index;
    }
}
