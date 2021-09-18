use crate::pkrd::reader::{Gen6Reader, Reader};

pub(super) struct PokemonORASReader<'a> {
    heap: &'a [u8],
}

impl<'a> PokemonORASReader<'a> {
    pub fn new(heap: &'a [u8]) -> Self {
        Self { heap }
    }
}

impl<'a> Reader for PokemonORASReader<'a> {
    fn get_data(&self) -> &[u8] {
        self.heap
    }
}

impl<'a> Gen6Reader for PokemonORASReader<'a> {
    const INITIAL_SEED_OFFSET: usize = 0xc59e40;
    const MT_STATE_INDEX_OFFSET: usize = 0xc59e44;
    const TINYMT_STATE_OFFSET: usize = 0xC59E04;
    const PARTY_OFFSET: usize = 0xCFB26C;
    const EGG_READY_OFFSET: usize = 0xC88358;
    const EGG_OFFSET: usize = 0xC88360;
    const PARENT1_OFFSET: usize = 0xC88180;
    const PARENT2_OFFSET: usize = 0xC88270;
}
