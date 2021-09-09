use crate::pkrd::reader::{Gen7Reader, Reader};

pub(super) struct PokemonUSUMReader<'a> {
    heap: &'a [u8],
}

impl<'a> PokemonUSUMReader<'a> {
    pub fn new(heap: &'a [u8]) -> Self {
        Self { heap }
    }
}

impl<'a> Reader for PokemonUSUMReader<'a> {
    fn get_data(&self) -> &[u8] {
        self.heap
    }
}

impl<'a> Gen7Reader for PokemonUSUMReader<'a> {
    const INITIAL_SEED_OFFSET: usize = 0x2663BF0;
}
