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
    const MT_STATE_INDEX_OFFSET: usize = 0xc59e44;
    const PARTY_OFFSET: usize = 0xCFB26C;
}
