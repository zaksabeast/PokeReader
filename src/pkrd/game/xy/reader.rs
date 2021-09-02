use crate::pkrd::game::Reader;
use ctr::res::CtrResult;

pub(super) struct PokemonXYReader<'a> {
    heap: Reader<'a>,
}

impl<'a> PokemonXYReader<'a> {
    const MT_STATE_INDEX_OFFSET: usize = 0xc52848;

    pub fn new(heap: Reader<'a>) -> Self {
        Self { heap }
    }

    pub fn get_mt_state_index(&self) -> CtrResult<usize> {
        self.heap.read(Self::MT_STATE_INDEX_OFFSET)
    }
}
