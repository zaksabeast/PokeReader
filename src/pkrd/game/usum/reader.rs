use crate::pkrd::reader::Gen7Reader;
use no_std_io::Reader;

pub(super) struct PokemonUSUMReader {
    heap: &'static [u8],
}

impl PokemonUSUMReader {
    pub fn new(heap: &'static [u8]) -> Self {
        Self { heap }
    }
}

impl Reader for PokemonUSUMReader {
    fn get_slice(&self) -> &[u8] {
        self.heap
    }
}

impl Gen7Reader for PokemonUSUMReader {
    const INITIAL_SEED_OFFSET: usize = 0x2663BF0;
    const SFMT_STATE_INDEX_OFFSET: usize = 0x30D3F98;
    const SFMT_STATE_OFFSET: usize = 0x30D35D8;
    const PARTY_OFFSET: usize = 0x3F7FA44;
    const WILD_OFFSET: usize = 0x002F9A0;
    const SOS_OFFSET: usize = 0x002F9A0;
    const SOS_SEED_OFFSET: usize = 0x0038E30;
    const SOS_CHAIN_LENGTH: usize = 0x00397F9;
    const PELAGO_OFFSET_1: usize = 0x304D342;
    const PELAGO_OFFSET_2: usize = 0x304D256;
    const PELAGO_OFFSET_3: usize = 0x304D16A;
    const EGG_READY_OFFSET: usize = 0x307B1E8;
    const EGG_OFFSET: usize = 0x307B1EC;
    const PARENT1_OFFSET: usize = 0x307B011;
    const PARENT2_OFFSET: usize = 0x307B0FA;
    const IS_PARENT1_OCCUPIED_OFFSET: usize = 0x307B010;
    const IS_PARENT2_OCCUPIED_OFFSET: usize = 0x307B0F9;
    const SHINY_CHARM_OFFSET: usize = 0x3012008;
    const WILD_TITLE: &'static str = "Wild";
    const SOS_TITLE: &'static str = "SOS";
    const PELAGO_TITLE_1: &'static str = "Pelago Slot 1";
    const PELAGO_TITLE_2: &'static str = "Pelago Slot 2";
    const PELAGO_TITLE_3: &'static str = "Pelago Slot 3";
}
