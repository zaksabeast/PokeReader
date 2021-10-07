use super::sfmt;
use crate::{log, pkrd::reader};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Gen7Rng {
    init_seed: u32,
    sfmt_rng: sfmt::SFMT,
    sfmt_advances: u32,
    sfmt_current_state: u64,
}

impl Gen7Rng {
    pub fn get_sfmt_advances(&self) -> u32 {
        self.sfmt_advances
    }

    fn update_sfmt(&mut self, sfmt_current_state: u64) {
        let mut temp_sfmt_current_state = self.sfmt_current_state;
        let mut is_state_found = false;

        // A boundary of 9999 makes sure we can't go in an infinite loop
        for advances in 0..9999 {
            if sfmt_current_state == temp_sfmt_current_state {
                self.sfmt_current_state = temp_sfmt_current_state;
                self.sfmt_advances += advances;
                is_state_found = true;
                break;
            }

            temp_sfmt_current_state = self.sfmt_rng.next();
        }

        if is_state_found == false {
            log::error(&alloc::format!(
                "SFMT State not found! Seed {:x}, State {:x}, Advances {}",
                self.init_seed,
                sfmt_current_state,
                self.sfmt_advances
            ));
        }
    }

    pub fn update(&mut self, game: &impl reader::Gen7Reader) {
        let sfmt_state = game.get_sfmt_state();
        let init_seed = game.get_initial_seed();

        if self.init_seed != init_seed && init_seed != 0 {
            self.init_seed = init_seed;
            self.sfmt_rng = sfmt::SFMT::new(init_seed);
            self.sfmt_advances = 0;
            self.sfmt_current_state = self.sfmt_rng.get_current_state();
        }

        self.update_sfmt(sfmt_state);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use mocktopus::mocking::{MockResult, Mockable};

    struct MockGen7Game {
        data: [u8; 0],
    }

    impl Default for MockGen7Game {
        fn default() -> Self {
            Self { data: [] }
        }
    }

    impl reader::Reader for MockGen7Game {
        fn get_data(&self) -> &[u8] {
            &self.data
        }
    }

    impl reader::Gen7Reader for MockGen7Game {
        const INITIAL_SEED_OFFSET: usize = 0;
        const SFMT_STATE_INDEX_OFFSET: usize = 0;
        const SFMT_STATE_OFFSET: usize = 0;
        const PARTY_OFFSET: usize = 0;
        const WILD_OFFSET: usize = 0;
        const SOS_OFFSET: usize = 0;
        const SOS_SEED_OFFSET: usize = 0;
        const SOS_CHAIN_LENGTH: usize = 0;
        const EGG_READY_OFFSET: usize = 0;
        const EGG_OFFSET: usize = 0;
        const PARENT1_OFFSET: usize = 0;
        const PARENT2_OFFSET: usize = 0;
        const IS_PARENT1_OCCUPIED_OFFSET: usize = 0;
        const IS_PARENT2_OCCUPIED_OFFSET: usize = 0;
        const SHINY_CHARM_OFFSET: usize = 0;
    }

    mod update_sfmt {
        use super::*;

        #[test]
        fn should_update_sfmt_info() {
            let mut rng = Gen7Rng::default();

            rng.sfmt_rng = sfmt::SFMT::new(0x7725e5e1);
            rng.update_sfmt(0xd7efa47e23000ac8);

            assert_eq!(rng.sfmt_advances, 1001);
            assert_eq!(rng.sfmt_current_state, 0xd7efa47e23000ac8);
        }
    }

    mod update {
        use super::*;

        #[test]
        fn should_reinitialize_values_if_sfmt_seed_changes() {
            let game = MockGen7Game::default();

            // Initial values
            reader::Gen7Reader::get_initial_seed
                .mock_safe(|_: &MockGen7Game| MockResult::Return(0x7725e5e1));
            reader::Gen7Reader::get_sfmt_state
                .mock_safe(|_: &MockGen7Game| MockResult::Return(0xd7efa47e23000ac8));

            let mut rng = Gen7Rng::default();
            rng.update(&game);

            // New values
            reader::Gen7Reader::get_initial_seed
                .mock_safe(|_: &MockGen7Game| MockResult::Return(0xc91cc389));
            reader::Gen7Reader::get_sfmt_state
                .mock_safe(|_: &MockGen7Game| MockResult::Return(0xb5618d99ce90d534));

            rng.update(&game);

            assert_eq!(rng.init_seed, 0xc91cc389);
            assert_eq!(rng.sfmt_current_state, 0xb5618d99ce90d534);
            assert_eq!(rng.sfmt_advances, 625);
        }

        #[test]
        fn should_not_reinitialize_values_if_sfmt_seed_does_not_change() {
            let game = MockGen7Game::default();

            // Initial values
            reader::Gen7Reader::get_initial_seed
                .mock_safe(|_: &MockGen7Game| MockResult::Return(0x7725e5e1));
            reader::Gen7Reader::get_sfmt_state
                .mock_safe(|_: &MockGen7Game| MockResult::Return(0x1947bef586be83e3));

            let mut rng = Gen7Rng::default();
            rng.update(&game);

            // New values
            reader::Gen7Reader::get_sfmt_state
                .mock_safe(|_: &MockGen7Game| MockResult::Return(0xd7efa47e23000ac8));

            rng.update(&game);

            assert_eq!(rng.init_seed, 0x7725e5e1);
            assert_eq!(rng.sfmt_current_state, 0xd7efa47e23000ac8);
            assert_eq!(rng.sfmt_advances, 1001);
        }
    }
}
