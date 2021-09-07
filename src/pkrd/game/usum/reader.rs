use crate::pkrd::reader::{Gen7Reader, Reader};
use ctr::res::CtrResult;
use safe_transmute::TriviallyTransmutable;

pub(super) struct PokemonUSUMReader<'a> {
    heap: Reader<'a>,
}

impl<'a> PokemonUSUMReader<'a> {
    pub fn new(heap: Reader<'a>) -> Self {
        Self { heap }
    }
}

impl<'a> Gen7Reader for PokemonUSUMReader<'a> {
    const INITIAL_SEED_OFFSET: usize = 0x2663BF0;

    fn read<T: TriviallyTransmutable>(&self, offset: usize) -> CtrResult<T> {
        self.heap.read(offset)
    }
}
