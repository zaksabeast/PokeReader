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
    const IS_PARENT1_OCCUPIED_OFFSET: usize;
    const IS_PARENT2_OCCUPIED_OFFSET: usize;

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

    fn get_egg_seed(&self) -> [u32; 4] {
        self.default_read(Self::EGG_OFFSET)
    }

    fn get_is_egg_ready(&self) -> bool {
        self.default_read::<u8>(Self::EGG_READY_OFFSET) != 0
    }

    fn get_egg_parent(&self, is_present_offset: usize, pkm_offset: usize) -> Option<pkm::Pk6> {
        let is_parent_present = self.default_read::<u8>(is_present_offset) != 0;

        if !is_parent_present {
            return None;
        }

        let parent = self.default_read::<pkm::Pk6Data>(pkm_offset).into();
        Some(parent)
    }

    fn get_egg_parent_1(&self) -> Option<pkm::Pk6> {
        self.get_egg_parent(Self::IS_PARENT1_OCCUPIED_OFFSET, Self::PARENT1_OFFSET)
    }

    fn get_egg_parent_2(&self) -> Option<pkm::Pk6> {
        self.get_egg_parent(Self::IS_PARENT2_OCCUPIED_OFFSET, Self::PARENT2_OFFSET)
    }
}
