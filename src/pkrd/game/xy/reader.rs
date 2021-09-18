use crate::pkrd::reader::{Gen6Reader, Reader};

pub(super) struct PokemonXYReader<'a> {
    heap: &'a [u8],
}

impl<'a> PokemonXYReader<'a> {
    pub fn new(heap: &'a [u8]) -> Self {
        Self { heap }
    }
}

impl<'a> Reader for PokemonXYReader<'a> {
    fn get_data(&self) -> &[u8] {
        self.heap
    }
}

impl<'a> Gen6Reader for PokemonXYReader<'a> {
    const INITIAL_SEED_OFFSET: usize = 0xc52844;
    const MT_STATE_INDEX_OFFSET: usize = 0xc52848;
    const TINYMT_STATE_OFFSET: usize = 0xc52808;
    const PARTY_OFFSET: usize = 0xCE1CF8;
    const EGG_READY_OFFSET: usize = 0xC80124;
    const EGG_OFFSET: usize = 0xC8012C;
    const PARENT1_OFFSET: usize = 0xC7FF4C;
    const PARENT2_OFFSET: usize = 0xC8003C;
}
