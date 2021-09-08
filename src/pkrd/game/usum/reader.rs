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

    const PARTY_ADDRESS_OFFSET: usize = 0x3F7FA44;
    const WILD_ADDRESS_OFFSET: usize = 0x002F9A0;
    const SOS_ADDRESS_OFFSET: usize = 0x002F9A0;

    const SOS_SEED_ADDRESS: usize = 0x0038E30;

    const SOS_CHAIN_LENGTH: usize = 0x00397F9;
    const EGG_READY_ADDRESS: usize = 0x307B1E8;
    const EGG_ADDRESS: usize = 0x307B1EC;
    const PARENT1_ADDRESS: usize = 0x307B011;
    const PARENT2_ADDRESS: usize = 0x307B0FA;

    fn read<T: TriviallyTransmutable>(&self, offset: usize) -> CtrResult<T> {
        self.heap.read(offset)
    }
}
