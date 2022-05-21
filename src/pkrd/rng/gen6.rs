use super::{mt, tinymt};
use crate::{log, pkrd::reader};

#[derive(Clone, Debug, PartialEq, Default)]
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

    fn update_mt(&mut self, mt_state: u32) {
        let mut temp_mt_state = self.mt_state;
        let mut is_state_found = false;

        // A boundary of 9999 makes sure we can't go in an infinite loop
        for advances in 0..9999 {
            if mt_state == temp_mt_state {
                self.mt_state = temp_mt_state;
                self.mt_advances += advances;
                is_state_found = true;
                break;
            }
            temp_mt_state = self.mt_rng.next();
        }

        if !is_state_found {
            log::error(&alloc::format!(
                "MT State not found! Seed {:x}, State {:x}, Advances {}",
                self.init_seed,
                mt_state,
                self.mt_advances
            ));
        }
    }

    fn update_tinymt(&mut self, tinymt_state: [u32; 4]) {
        let mut is_state_found = false;

        // A boundary of 9999 makes sure we can't go in an infinite loop
        for advances in 0..9999 {
            if tinymt_state == self.tinymt_rng.get_state() {
                self.tinymt_advances += advances;
                is_state_found = true;
                break;
            }
            self.tinymt_rng.next_state();
        }

        if !is_state_found {
            log::error(&alloc::format!(
                "TinyMT State not found! InitialState[0] {:x}, InitialState[1] {:x}, InitialState[2] {:x}, InitialState[3] {:x}, State[0] {:x}, State[1] {:x}, State[2] {:x}, State[3] {:x}, Advances {}",
                self.init_tinymt_state[0],
                self.init_tinymt_state[1],
                self.init_tinymt_state[2],
                self.init_tinymt_state[3],
                tinymt_state[0],
                tinymt_state[1],
                tinymt_state[2],
                tinymt_state[3],
                self.tinymt_advances
            ));
        }
    }

    pub fn update(&mut self, game: &impl reader::Gen6Reader) {
        let mt_state = game.get_mt_state();
        let init_seed = game.get_initial_seed();
        let tinymt_state = game.get_tinymt_state();

        if self.init_seed != init_seed && init_seed != 0 {
            self.mt_rng = mt::MT::new(init_seed);
            self.tinymt_rng = tinymt::TinyMT::new(tinymt_state);
            self.mt_advances = 0;
            self.tinymt_advances = 0;
            self.init_seed = init_seed;
            self.init_tinymt_state = tinymt_state;
            self.mt_state = init_seed;
        }

        self.update_mt(mt_state);
        self.update_tinymt(tinymt_state);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use mocktopus::mocking::{MockResult, Mockable};
    use no_std_io::Reader;

    struct MockGen6Game {
        data: [u8; 0],
    }

    impl Default for MockGen6Game {
        fn default() -> Self {
            Self { data: [] }
        }
    }

    impl Reader for MockGen6Game {
        fn get_slice(&self) -> &[u8] {
            &self.data
        }
    }

    impl reader::Gen6Reader for MockGen6Game {
        const INITIAL_SEED_OFFSET: usize = 0;
        const MT_START_OFFSET: usize = 0;
        const MT_STATE_INDEX_OFFSET: usize = 0;
        const TINYMT_STATE_OFFSET: usize = 0;
        const PARTY_OFFSET: usize = 0;
        const EGG_READY_OFFSET: usize = 0;
        const EGG_OFFSET: usize = 0;
        const PARENT1_OFFSET: usize = 0;
        const PARENT2_OFFSET: usize = 0;
        const IS_PARENT1_OCCUPIED_OFFSET: usize = 0;
        const IS_PARENT2_OCCUPIED_OFFSET: usize = 0;
    }

    mod update_mt {
        use super::*;

        #[test]
        fn should_update_mt_info() {
            let mut rng = Gen6Rng::default();

            rng.mt_rng = mt::MT::new(0xaabbccdd);
            rng.update_mt(0xd80fcb47);

            assert_eq!(rng.mt_advances, 625);
            assert_eq!(rng.mt_state, 0xd80fcb47);
        }
    }

    mod update_tinymt {
        use super::*;

        #[test]
        fn should_update_tinymt_info() {
            let mut rng = Gen6Rng::default();

            rng.tinymt_rng = tinymt::TinyMT::new([0x11112222, 0x33334444, 0x55556666, 0x77778888]);
            rng.update_tinymt([0x233f3c9d, 0x5a385202, 0x56e043c9, 0x76b46859]);

            assert_eq!(rng.tinymt_advances, 156);
        }
    }

    mod update {
        use super::*;

        #[test]
        fn should_reinitialize_values_if_mt_seed_changes() {
            let game = MockGen6Game::default();

            // Initial values
            reader::Gen6Reader::get_initial_seed
                .mock_safe(|_: &MockGen6Game| MockResult::Return(0xaabbccdd));
            reader::Gen6Reader::get_mt_state
                .mock_safe(|_: &MockGen6Game| MockResult::Return(0xd80fcb47));
            reader::Gen6Reader::get_tinymt_state.mock_safe(|_: &MockGen6Game| {
                MockResult::Return([0x11111111, 0x11111111, 0x11111111, 0x11111111])
            });

            let mut rng = Gen6Rng::default();
            rng.update(&game);

            // New values
            reader::Gen6Reader::get_initial_seed
                .mock_safe(|_: &MockGen6Game| MockResult::Return(0x11111111));
            reader::Gen6Reader::get_mt_state
                .mock_safe(|_: &MockGen6Game| MockResult::Return(0x32151361));
            reader::Gen6Reader::get_tinymt_state.mock_safe(|_: &MockGen6Game| {
                MockResult::Return([0x22222222, 0x22222222, 0x22222222, 0x22222222])
            });

            rng.update(&game);

            assert_eq!(rng.init_seed, 0x11111111);
            assert_eq!(rng.mt_state, 0x32151361);
            assert_eq!(rng.mt_advances, 4);
            assert_eq!(rng.tinymt_advances, 0);
            assert_eq!(
                rng.init_tinymt_state,
                [0x22222222, 0x22222222, 0x22222222, 0x22222222]
            );
        }

        #[test]
        fn should_not_reinitialize_values_if_mt_seed_does_not_change() {
            let game = MockGen6Game::default();

            // Initial values
            reader::Gen6Reader::get_initial_seed
                .mock_safe(|_: &MockGen6Game| MockResult::Return(0xaabbccdd));
            reader::Gen6Reader::get_mt_state
                .mock_safe(|_: &MockGen6Game| MockResult::Return(0x9eecaded));
            reader::Gen6Reader::get_tinymt_state.mock_safe(|_: &MockGen6Game| {
                MockResult::Return([0x11111111, 0x11111111, 0x11111111, 0x11111111])
            });

            let mut rng = Gen6Rng::default();
            rng.update(&game);

            // New values
            reader::Gen6Reader::get_mt_state
                .mock_safe(|_: &MockGen6Game| MockResult::Return(0xd80fcb47));
            reader::Gen6Reader::get_tinymt_state.mock_safe(|_: &MockGen6Game| {
                MockResult::Return([0x11111111, 0x99999b33, 0xffe00555, 0x955552aa])
            });

            rng.update(&game);

            assert_eq!(rng.init_seed, 0xaabbccdd);
            assert_eq!(rng.mt_state, 0xd80fcb47);
            assert_eq!(rng.mt_advances, 625);
            assert_eq!(rng.tinymt_advances, 2);
            assert_eq!(
                rng.init_tinymt_state,
                [0x11111111, 0x11111111, 0x11111111, 0x11111111]
            );
        }
    }
}
