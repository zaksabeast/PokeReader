use crate::rng;

#[derive(Default)]
pub struct RngWrapper<T: rng::Rng + Copy> {
    rng: T,
    init_seed: T::Seed,
    advances: u32,
    current_state: T::CurrentState,
}

impl<T: rng::Rng + Copy> RngWrapper<T> {
    pub fn init_seed(&self) -> T::Seed {
        self.init_seed
    }

    pub fn current_state(&self) -> T::CurrentState {
        self.current_state
    }

    pub fn set_current_state(&mut self, current_state: T::CurrentState) {
        self.current_state = current_state;
    }

    pub fn advances(&self) -> u32 {
        self.advances
    }

    pub fn reinit(&mut self, seed: T::Seed) {
        self.rng = T::new(seed);
        self.init_seed = seed;
        self.advances = 0;
    }

    pub fn reinit_if_needed(&mut self, seed: T::Seed) -> bool {
        if self.init_seed != seed {
            self.reinit(seed);
            return true;
        }

        false
    }

    pub fn update_advances(&mut self, current_state: T::CurrentState) {
        if self.init_seed == T::Seed::default() || current_state == self.current_state {
            return;
        }

        // People really do go for this many advances, and even higher
        for advances in 1..1_000_000 {
            if current_state == self.rng.next_state() {
                self.advances += advances;
                self.current_state = current_state;
                return;
            }
        }
    }
}
