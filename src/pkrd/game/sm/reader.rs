use crate::pkrd::reader::{Gen7Reader, Reader};
use ctr::res::CtrResult;
use safe_transmute::TriviallyTransmutable;

pub(super) struct PokemonSMReader<'a> {
    heap: Reader<'a>,
}

impl<'a> PokemonSMReader<'a> {
    pub fn new(heap: Reader<'a>) -> Self {
        Self { heap }
    }
}

impl<'a> Gen7Reader for PokemonSMReader<'a> {
    const INITIAL_SEED_OFFSET: usize = 0x25A3878;

    fn read<T: TriviallyTransmutable>(&self, offset: usize) -> CtrResult<T> {
        self.heap.read(offset)
    }
}
