use super::mt;
use super::tinymt;
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Gen6Rng {
    init_seed: u32,
    init_tinymt_state: [u32; 4],
    mt_rng: mt::MT,
    tinymt_rng: tinymt::TinyMT,
    mt_advances: u32,
    tinymt_advances: u32,
    mt_state: u32,
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

    pub fn get_mt_state(&self) -> u32 {
        self.mt_state
    }

    pub fn update(&mut self, mt_state: u32, init_seed: u32, tinymt_state: [u32; 4]) {
        if self.init_seed != init_seed && init_seed != 0 {
            self.mt_rng = mt::MT::new(init_seed);
            self.tinymt_rng = tinymt::TinyMT::new(tinymt_state);
            self.mt_advances = 0;
            self.tinymt_advances = 0;
            self.init_seed = init_seed;
            self.init_tinymt_state = tinymt_state;
            self.mt_state = init_seed;
        }

        // A boundary of 9999 makes sure we can't go in an infinite loop
        let mut temp_mt_state = self.mt_state;
        for advances in 0..9999 {
            if mt_state == temp_mt_state || mt_state == 0 {
                self.mt_state = temp_mt_state;
                self.mt_advances += advances;
                break;
            }
            temp_mt_state = self.mt_rng.next();
        }

        // Same as above, 9999 prevents infinite loop
        for tinymt_advances in 0..9999 {
            if tinymt_state == self.get_tinymt_state() || tinymt_state == [0, 0, 0, 0] {
                self.tinymt_advances += tinymt_advances;
                break;
            }
            self.tinymt_rng.next_state();
        }
    }
}
