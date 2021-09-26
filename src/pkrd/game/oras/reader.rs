use crate::pkrd::reader::{Gen6Reader, Reader};

pub(super) struct PokemonORASReader {
    heap: &'static [u8],
}

impl PokemonORASReader {
    pub fn new(heap: &'static [u8]) -> Self {
        Self { heap }
    }
}

impl Reader for PokemonORASReader {
    fn get_data(&self) -> &[u8] {
        self.heap
    }
}

impl Gen6Reader for PokemonORASReader {
    const INITIAL_SEED_OFFSET: usize = 0xc59e40;
    const MT_START_OFFSET: usize = 0xc59e48;
    const MT_STATE_INDEX_OFFSET: usize = 0xc59e44;
    const TINYMT_STATE_OFFSET: usize = 0xC59E04;
    const PARTY_OFFSET: usize = 0xCFB26C;
    const EGG_READY_OFFSET: usize = 0xC88358;
    const EGG_OFFSET: usize = 0xC88360;
    const PARENT1_OFFSET: usize = 0xC88180;
    const PARENT2_OFFSET: usize = 0xC88270;
}
