use super::reader::TransporterReader;
use crate::rng::{RngWrapper, MT};

#[derive(Default)]
pub struct TransporterRng {
    mt: RngWrapper<MT>,
}

impl TransporterRng {
    pub fn update(&mut self, reader: &TransporterReader) {
        let init_seed = reader.initial_seed();
        let mt_state = reader.mt_state();

        self.mt.reinit_if_needed(init_seed);
        self.mt.update_advances(mt_state);
        self.mt.set_current_state(mt_state);
    }

    pub fn mt(&self) -> &RngWrapper<MT> {
        &self.mt
    }
}
