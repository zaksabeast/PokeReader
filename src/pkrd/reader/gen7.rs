use super::{pkm, Reader};

pub trait Gen7Reader: Reader {
    const INITIAL_SEED_OFFSET: usize;
    const PARTY_OFFSET: usize;
    const WILD_OFFSET: usize;
    const SOS_OFFSET: usize;
    const SOS_SEED_OFFSET: usize;
    const SOS_CHAIN_LENGTH: usize;
    const EGG_READY_OFFSET: usize;
    const EGG_OFFSET: usize;
    const PARENT1_OFFSET: usize;
    const PARENT2_OFFSET: usize;

    fn get_initial_seed(&self) -> u32 {
        self.default_read(Self::INITIAL_SEED_OFFSET)
    }

    fn get_party_pkm(&self, slot: pkm::PartySlot) -> pkm::Pk7 {
        let offset = ((slot as usize) * 484) + Self::PARTY_OFFSET;
        self.default_read::<pkm::Pk7Data>(offset).into()
    }

    fn get_wild_pkm(&self) -> pkm::Pk7 {
        self.default_read::<pkm::Pk7Data>(Self::WILD_OFFSET).into()
    }
}
