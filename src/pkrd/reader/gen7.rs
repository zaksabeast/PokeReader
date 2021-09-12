use super::{pkm, Reader};

pub trait Gen7Reader: Reader {
    const INITIAL_SEED_OFFSET: usize;
    const PARTY_OFFSET: usize;

    fn get_initial_seed(&self) -> u32 {
        self.default_read(Self::INITIAL_SEED_OFFSET)
    }

    fn get_party_pkm(&self, slot: pkm::PartySlot) -> pkm::Pk7 {
        let offset = ((slot as usize) * 484) + Self::PARTY_OFFSET;
        self.default_read::<pkm::Pk7Data>(offset).into()
    }
}
