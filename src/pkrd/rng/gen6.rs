use super::mt;
use super::tinymt;
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Gen6Rng {
    init_seed: usize,
    init_tinymt_state: [u32; 4],
    mt_rng: mt::MT,
    tinymt_rng: tinymt::TinyMT,
    mt_advances: u32,
    tinymt_advances: u32,
    mt_seed: u32,
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

    pub fn get_mt_seed(&self) -> u32 {
        self.mt_seed
    }

    pub fn update(&mut self, mt_seed: usize, init_seed: usize, tinymt_state: [u32; 4]) {
        if self.init_seed != init_seed && init_seed != 0 {
            self.mt_rng = mt::MT::new(init_seed as u32);
            self.tinymt_rng = tinymt::TinyMT::new(tinymt_state);
            self.mt_advances = 0;
            self.tinymt_advances = 0;
            self.init_seed = init_seed;
            self.init_tinymt_state = tinymt_state;
            self.mt_seed = init_seed as u32;
        }

        let mut temp_seed = self.mt_seed;
        let mut temp_advances = 0;

        while (mt_seed as u32) != temp_seed {
            temp_seed = self.mt_rng.next();
            temp_advances += 1;
            if temp_advances > 9999 {
                temp_seed = self.mt_seed;
                temp_advances = 0;
                break;
            }
        }
        self.mt_seed = temp_seed;
        self.mt_advances += temp_advances;

        while tinymt_state != self.get_tinymt_state() && tinymt_state != [0, 0, 0, 0] {
            self.tinymt_rng.next_state();
            self.tinymt_advances += 1;
        }
    }
}
