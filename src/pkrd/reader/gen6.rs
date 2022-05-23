use crate::utils::party_slot::PartySlot;
use crate::utils::CircularCounter;
use no_std_io::Reader;
use pkm_rs::pkm;

pub struct Daycare {
    pub daycare_title: &'static str,
    pub daycare_footer: &'static str,
    pub egg_seed: [u32; 4],
    pub is_egg_ready: bool,
    pub parent_1: Option<pkm::Pk6>,
    pub parent_2: Option<pkm::Pk6>,
}

pub type DaycareSlot = CircularCounter<0, 1>;

#[cfg_attr(not(target_os = "horizon"), mocktopus::macros::mockable)]
pub trait Gen6Reader: Reader {
    const INITIAL_SEED_OFFSET: usize;
    const MT_START_OFFSET: usize;
    const MT_STATE_INDEX_OFFSET: usize;
    const TINYMT_STATE_OFFSET: usize;
    const PARTY_OFFSET: usize;
    const EGG_READY_OFFSET_1: usize;
    const EGG_SEED_OFFSET_1: usize;
    const PARENT1_OFFSET_1: usize;
    const PARENT2_OFFSET_1: usize;
    const IS_PARENT1_OCCUPIED_OFFSET_1: usize;
    const IS_PARENT2_OCCUPIED_OFFSET_1: usize;
    const DAYCARE_TITLE_1: &'static str;
    const DAYCARE_FOOTER_1: &'static str;
    const EGG_READY_OFFSET_2: usize;
    const EGG_SEED_OFFSET_2: usize;
    const PARENT1_OFFSET_2: usize;
    const PARENT2_OFFSET_2: usize;
    const IS_PARENT1_OCCUPIED_OFFSET_2: usize;
    const IS_PARENT2_OCCUPIED_OFFSET_2: usize;
    const DAYCARE_TITLE_2: &'static str;
    const DAYCARE_FOOTER_2: &'static str;

    fn get_daycare(&self, daycare_slot: DaycareSlot) -> Daycare {
        if daycare_slot.value() == 0 {
            Daycare {
                daycare_title: Self::DAYCARE_TITLE_1,
                daycare_footer: Self::DAYCARE_FOOTER_1,
                egg_seed: self.default_read(Self::EGG_SEED_OFFSET_1),
                is_egg_ready: self.default_read::<u8>(Self::EGG_READY_OFFSET_1) != 0,
                parent_1: self
                    .get_egg_parent(Self::IS_PARENT1_OCCUPIED_OFFSET_1, Self::PARENT1_OFFSET_1),
                parent_2: self
                    .get_egg_parent(Self::IS_PARENT2_OCCUPIED_OFFSET_1, Self::PARENT2_OFFSET_1),
            }
        } else {
            Daycare {
                daycare_title: Self::DAYCARE_TITLE_2,
                daycare_footer: Self::DAYCARE_FOOTER_2,
                egg_seed: self.default_read(Self::EGG_SEED_OFFSET_2),
                is_egg_ready: self.default_read::<u8>(Self::EGG_READY_OFFSET_2) != 0,
                parent_1: self
                    .get_egg_parent(Self::IS_PARENT1_OCCUPIED_OFFSET_2, Self::PARENT1_OFFSET_2),
                parent_2: self
                    .get_egg_parent(Self::IS_PARENT2_OCCUPIED_OFFSET_2, Self::PARENT2_OFFSET_2),
            }
        }
    }

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
        let offset = ((slot.value() as usize - 1) * 484) + Self::PARTY_OFFSET;
        self.default_read::<pkm::Pk6Data>(offset).into()
    }

    fn get_egg_parent(&self, is_present_offset: usize, pkm_offset: usize) -> Option<pkm::Pk6> {
        let is_parent_present = self.default_read::<u8>(is_present_offset) != 0;

        if !is_parent_present {
            return None;
        }

        let parent = self.default_read::<pkm::Pk6Data>(pkm_offset).into();
        Some(parent)
    }
}
