use crate::pkrd::reader::Gen6Reader;
use no_std_io::Reader;

pub(super) struct PokemonORASReader {
    heap: &'static [u8],
}

impl PokemonORASReader {
    pub fn new(heap: &'static [u8]) -> Self {
        Self { heap }
    }
}

impl Reader for PokemonORASReader {
    fn get_slice(&self) -> &[u8] {
        self.heap
    }
}

impl Gen6Reader for PokemonORASReader {
    const INITIAL_SEED_OFFSET: usize = 0xc59e40;
    const MT_START_OFFSET: usize = 0xc59e48;
    const MT_STATE_INDEX_OFFSET: usize = 0xc59e44;
    const TINYMT_STATE_OFFSET: usize = 0xC59E04;
    const PARTY_OFFSET: usize = 0xCFB26C;
    const EGG_READY_OFFSET: usize = 0xC88358;
    const EGG_SEED_OFFSET: usize = 0xC88360;
    const PARENT1_OFFSET: usize = 0xC88180;
    const PARENT2_OFFSET: usize = 0xC88270;
    const IS_PARENT1_OCCUPIED_OFFSET: usize = 0xC88178;
    const IS_PARENT2_OCCUPIED_OFFSET: usize = 0xC88268;
    const DAYCARE_TITLE: &'static str = "Daycare: Route 117";
    const DAYCARE_FOOTER: &'static str = "Select + Right ->";
    const EGG_READY_OFFSET_2: usize = 0xC88548;
    const EGG_SEED_OFFSET_2: usize = 0xC88550;
    const PARENT1_OFFSET_2: usize = 0xC88370;
    const PARENT2_OFFSET_2: usize = 0xC88460;
    const IS_PARENT1_OCCUPIED_OFFSET_2: usize = 0xC88368;
    const IS_PARENT2_OCCUPIED_OFFSET_2: usize = 0xC88458;
    const DAYCARE_TITLE_2: &'static str = "Daycare: Battle Resort";
    const DAYCARE_FOOTER_2: &'static str = "<- Select + Left";
}
