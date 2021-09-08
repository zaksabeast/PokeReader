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

    const PARTY_ADDRESS: usize = 0x4195E10;
    const WILD_ADDRESS: usize = 0x002F7B8;
    const SOS_ADDRESS: usize = 0x002F7B8;

    const SOS_SEED_ADDRESS: usize = 0x0038C44;
    const SOS_CHAIN_LENGTH: usize = 0x003960D;

    const EGG_READY: usize = 0x313EDD8;
    const EGG_ADDRESS: usize = 0x313EDDC;
    const PARENT1_ADDRESS: usize = 0x313EC01;
    const PARENT2_ADDRESS: usize = 0x313ECEA;

    fn read<T: TriviallyTransmutable>(&self, offset: usize) -> CtrResult<T> {
        self.heap.read(offset)
    }
}
