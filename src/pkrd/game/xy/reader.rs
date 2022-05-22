use crate::pkrd::reader::Gen6Reader;
use no_std_io::Reader;

pub(super) struct PokemonXYReader {
    heap: &'static [u8],
}

impl PokemonXYReader {
    pub fn new(heap: &'static [u8]) -> Self {
        Self { heap }
    }
}

impl Reader for PokemonXYReader {
    fn get_slice(&self) -> &[u8] {
        self.heap
    }
}

impl Gen6Reader for PokemonXYReader {
    const INITIAL_SEED_OFFSET: usize = 0xc52844;
    const MT_START_OFFSET: usize = 0xc5284c;
    const MT_STATE_INDEX_OFFSET: usize = 0xc52848;
    const TINYMT_STATE_OFFSET: usize = 0xc52808;
    const PARTY_OFFSET: usize = 0xCE1CF8;
    const EGG_READY_OFFSET: usize = 0xC80124;
    const EGG_OFFSET: usize = 0xC8012C;
    const PARENT1_OFFSET: usize = 0xC7FF4C;
    const PARENT2_OFFSET: usize = 0xC8003C;
    const IS_PARENT1_OCCUPIED_OFFSET: usize = 0xC7FF44;
    const IS_PARENT2_OCCUPIED_OFFSET: usize = 0xC80034;
    const DAYCARE_TITLE: &'static str = "Daycare View";
}
