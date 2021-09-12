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
    const MT_STATE_INDEX_OFFSET: usize = 0xc52848;
    const PARTY_OFFSET: usize = 0xCE1CF8;
}
