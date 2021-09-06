use crate::pkrd::reader::Reader;
use ctr::res::CtrResult;

pub(super) struct PokemonUSUMReader<'a> {
    heap: Reader<'a>,
}

impl<'a> PokemonUSUMReader<'a> {
    const INITIAL_SEED_OFFSET: usize = 0x2663BF0;

    pub fn new(heap: Reader<'a>) -> Self {
        Self { heap }
    }

    pub fn get_initial_seed(&self) -> CtrResult<u32> {
        self.heap.read(Self::INITIAL_SEED_OFFSET)
    }
}
