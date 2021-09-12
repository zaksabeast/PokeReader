use crate::pkrd::reader::{Gen7Reader, Reader};

pub(super) struct PokemonUSUMReader<'a> {
    heap: &'a [u8],
}

impl<'a> PokemonUSUMReader<'a> {
    pub fn new(heap: &'a [u8]) -> Self {
        Self { heap }
    }
}

impl<'a> Reader for PokemonUSUMReader<'a> {
    fn get_data(&self) -> &[u8] {
        self.heap
    }
}

impl<'a> Gen7Reader for PokemonUSUMReader<'a> {
    const INITIAL_SEED_OFFSET: usize = 0x2663BF0;
    const PARTY_OFFSET: usize = 0x3F7FA44;
    const WILD_OFFSET: usize = 0x002F9A0;
    const SOS_OFFSET: usize = 0x002F9A0;
    const SOS_SEED_OFFSET: usize = 0x0038E30;
    const SOS_CHAIN_LENGTH: usize = 0x00397F9;
    const EGG_READY_OFFSET: usize = 0x307B1E8;
    const EGG_OFFSET: usize = 0x307B1EC;
    const PARENT1_OFFSET: usize = 0x307B011;
    const PARENT2_OFFSET: usize = 0x307B0FA;
}
