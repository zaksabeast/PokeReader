use crate::pkrd::reader::{Gen6Reader, Reader};
use ctr::res::CtrResult;
use safe_transmute::TriviallyTransmutable;

pub(super) struct PokemonXYReader<'a> {
    heap: Reader<'a>,
}

impl<'a> PokemonXYReader<'a> {
    pub fn new(heap: Reader<'a>) -> Self {
        Self { heap }
    }
}

impl<'a> Gen6Reader for PokemonXYReader<'a> {
    const MT_STATE_INDEX_OFFSET: usize = 0xc52848;

    fn read<T: TriviallyTransmutable>(&self, offset: usize) -> CtrResult<T> {
        self.heap.read(offset)
    }
}
