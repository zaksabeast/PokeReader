use crate::pkrd::reader::{Gen7Reader, Reader};

pub(super) struct PokemonSMReader<'a> {
    heap: &'a [u8],
}

impl<'a> PokemonSMReader<'a> {
    pub fn new(heap: &'a [u8]) -> Self {
        Self { heap }
    }
}

impl<'a> Reader for PokemonSMReader<'a> {
    fn get_data(&self) -> &[u8] {
        self.heap
    }
}

impl<'a> Gen7Reader for PokemonSMReader<'a> {
    const INITIAL_SEED_OFFSET: usize = 0x25A3878;
}
