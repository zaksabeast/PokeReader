use crate::pkrd::reader::{Gen6Reader, Reader};
use ctr::res::CtrResult;
use safe_transmute::TriviallyTransmutable;

pub(super) struct PokemonORASReader<'a> {
    heap: Reader<'a>,
}

impl<'a> PokemonORASReader<'a> {
    pub fn new(heap: Reader<'a>) -> Self {
        Self { heap }
    }
}

impl<'a> Gen6Reader for PokemonORASReader<'a> {
    const MT_STATE_INDEX_OFFSET: usize = 0xc59e44;

    const PARTY_ADDRESS: usize = 0xCFB26C;

    const EGG_READY: usize = 0xC88358;
    const EGG_ADDRESS: usize = 0xC88360;
    const PARENT1_ADDRESS: usize = 0xC88180;
    const PARENT2_ADDRESS: usize = 0xC88270;

    fn read<T: TriviallyTransmutable>(&self, offset: usize) -> CtrResult<T> {
        self.heap.read(offset)
    }
}
