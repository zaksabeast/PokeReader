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

    const PARTY_ADDRESS: usize = 0xCE1CF8;

    const EGG_READY: usize = 0xC80124;
    const EGG_ADDRESS: usize = 0xC8012C;
    const PARENT1_ADDRESS: usize = 0xC7FF4C;
    const PARENT2_ADDRESS: usize = 0xC8003C;

    fn read<T: TriviallyTransmutable>(&self, offset: usize) -> CtrResult<T> {
        self.heap.read(offset)
    }
}
