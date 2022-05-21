use crate::utils::party_slot::PartySlot;
use no_std_io::Reader;
use pkm_rs::pkm;

#[cfg_attr(not(target_os = "horizon"), mocktopus::macros::mockable)]
pub trait Gen6Reader: Reader {
    const INITIAL_SEED_OFFSET: usize;
    const MT_START_OFFSET: usize;
    const MT_STATE_INDEX_OFFSET: usize;
    const TINYMT_STATE_OFFSET: usize;
    const PARTY_OFFSET: usize;
    const EGG_READY_OFFSET: usize;
    const EGG_OFFSET: usize;
    const PARENT1_OFFSET: usize;
    const PARENT2_OFFSET: usize;

    fn get_initial_seed(&self) -> u32 {
        self.default_read(Self::INITIAL_SEED_OFFSET)
    }

    fn get_mt_state_index(&self) -> usize {
        self.default_read(Self::MT_STATE_INDEX_OFFSET)
    }

    fn get_mt_state(&self) -> u32 {
        let index = self.get_mt_state_index();
        self.default_read(Self::MT_START_OFFSET + if index != 624 { index * 4 } else { 0 })
    }

    fn get_tinymt_state(&self) -> [u32; 4] {
        self.default_read(Self::TINYMT_STATE_OFFSET)
    }

    fn get_party_pkm(&self, slot: PartySlot) -> pkm::Pk6 {
        let offset = ((slot.value() as usize) * 484) + Self::PARTY_OFFSET;
        self.default_read::<pkm::Pk6Data>(offset).into()
    }
}
