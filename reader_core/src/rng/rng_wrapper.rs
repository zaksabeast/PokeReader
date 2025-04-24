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
        self.current_state = self.rng.current_state();
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

    pub fn rng(&self) -> &T {
        &self.rng
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rng::{Rng, Sfmt, TinyMT, MT};

    #[test]
    fn should_track_sfmt_advances() {
        let seed = 0x92845c35;
        let mut rng = Sfmt::new(seed);

        for _ in 0..478 {
            rng.next_state();
        }
        assert_eq!(rng.current_state(), 0x5a1ef513d10eccfb);

        let mut wrapper = RngWrapper::<Sfmt>::default();
        wrapper.reinit(seed);
        wrapper.update_advances(rng.current_state());
        assert_eq!(wrapper.advances(), 478);
    }

    fn should_track_mt_advances() {
        let seed = 0x4fa7c9da;
        let mut rng = MT::new(seed);

        for _ in 0..346 {
            rng.next_state();
        }
        assert_eq!(rng.current_state(), 0xc81fa608);

        let mut wrapper = RngWrapper::<MT>::default();
        wrapper.reinit(seed);
        wrapper.update_advances(rng.current_state());
        assert_eq!(wrapper.advances(), 346);
    }

    #[test]
    fn should_track_tinymt_advances() {
        let seed = 0xaabbccdd;
        let mut rng = TinyMT::new(seed);

        for _ in 0..15 {
            rng.next_state();
        }
        assert_eq!(
            rng.current_state(),
            [0x39d40c61, 0x1d3be6ca, 0xb3bed380, 0x8ac476ea]
        );

        let mut wrapper = RngWrapper::<TinyMT>::default();
        wrapper.reinit(seed);
        wrapper.update_advances(rng.current_state());
        assert_eq!(wrapper.advances(), 15);
    }
}
