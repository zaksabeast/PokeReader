use crate::{
    pkrd::views::WildPokemon,
    utils::{party_slot::PartySlot, CircularCounter},
};
use no_std_io::Reader;
use pkm_rs::pkm;

pub type WildSlot = CircularCounter<0, 4>;

pub type RngSlot = CircularCounter<0, 1>;

#[cfg_attr(not(target_os = "horizon"), mocktopus::macros::mockable)]
pub trait Gen7Reader: Reader {
    const INITIAL_SEED_OFFSET: usize;
    const SFMT_STATE_INDEX_OFFSET: usize;
    const SFMT_STATE_OFFSET: usize;
    const PARTY_OFFSET: usize;
    const WILD_OFFSET: usize;
    const SOS_OFFSET: usize;
    const SOS_SEED_OFFSET: usize;
    const SOS_CHAIN_LENGTH: usize;
    const EGG_READY_OFFSET: usize;
    const EGG_OFFSET: usize;
    const PELAGO_OFFSET_1: usize;
    const PELAGO_OFFSET_2: usize;
    const PELAGO_OFFSET_3: usize;
    const PARENT1_OFFSET: usize;
    const PARENT2_OFFSET: usize;
    const IS_PARENT1_OCCUPIED_OFFSET: usize;
    const IS_PARENT2_OCCUPIED_OFFSET: usize;
    const SHINY_CHARM_OFFSET: usize;
    const WILD_TITLE: &'static str;
    const SOS_TITLE: &'static str;
    const PELAGO_TITLE_1: &'static str;
    const PELAGO_TITLE_2: &'static str;
    const PELAGO_TITLE_3: &'static str;
    const ID_OFFSET: usize;

    fn get_tid(&self) -> u32 {
        let sidtid = self.default_read::<u32>(Self::ID_OFFSET);

        sidtid % 1000000
    }

    fn get_tsv(&self) -> u16 {
        let sid = self.default_read::<u16>(Self::ID_OFFSET);
        let tid = self.default_read::<u16>(Self::ID_OFFSET + 2);

        (tid ^ sid) >> 4
    }

    fn get_wild(&self, wild_slot: WildSlot) -> WildPokemon<pkm::Pk7> {
        match wild_slot.value() {
            1 => WildPokemon {
                title: Self::SOS_TITLE,
                pkx: self.default_read::<pkm::Pk7Data>(Self::SOS_OFFSET).into(),
            },
            2 => WildPokemon {
                title: Self::PELAGO_TITLE_1,
                pkx: self
                    .default_read::<pkm::Pk7Data>(Self::PELAGO_OFFSET_1)
                    .into(),
            },
            3 => WildPokemon {
                title: Self::PELAGO_TITLE_2,
                pkx: self
                    .default_read::<pkm::Pk7Data>(Self::PELAGO_OFFSET_2)
                    .into(),
            },
            4 => WildPokemon {
                title: Self::PELAGO_TITLE_3,
                pkx: self
                    .default_read::<pkm::Pk7Data>(Self::PELAGO_OFFSET_3)
                    .into(),
            },
            _ => WildPokemon {
                title: Self::WILD_TITLE,
                pkx: self.default_read::<pkm::Pk7Data>(Self::WILD_OFFSET).into(),
            },
        }
    }

    fn get_initial_seed(&self) -> u32 {
        self.default_read(Self::INITIAL_SEED_OFFSET)
    }

    fn get_sfmt_state_index(&self) -> usize {
        self.default_read(Self::SFMT_STATE_INDEX_OFFSET)
    }

    fn get_sfmt_state(&self) -> u64 {
        let index = self.get_sfmt_state_index();
        self.default_read(Self::SFMT_STATE_OFFSET + if index != 624 { index * 4 } else { 0 })
    }

    fn get_egg_seed(&self) -> [u32; 4] {
        self.default_read(Self::EGG_OFFSET)
    }

    fn get_sos_seed(&self) -> u32 {
        self.default_read(Self::SOS_SEED_OFFSET)
    }

    fn get_sos_chain(&self) -> u8 {
        self.default_read(Self::SOS_CHAIN_LENGTH)
    }

    fn get_party_pkm(&self, slot: PartySlot) -> pkm::Pk7 {
        let offset = ((slot.value() as usize) * 484) + Self::PARTY_OFFSET;
        self.default_read::<pkm::Pk7Data>(offset).into()
    }

    fn get_egg_parent(&self, is_present_offset: usize, pkm_offset: usize) -> Option<pkm::Pk7> {
        let is_parent_present = self.default_read::<u8>(is_present_offset) != 0;

        if !is_parent_present {
            return None;
        }

        let parent = self.default_read::<pkm::Pk7Data>(pkm_offset).into();
        Some(parent)
    }

    fn get_egg_parent_1(&self) -> Option<pkm::Pk7> {
        self.get_egg_parent(Self::IS_PARENT1_OCCUPIED_OFFSET, Self::PARENT1_OFFSET)
    }

    fn get_egg_parent_2(&self) -> Option<pkm::Pk7> {
        self.get_egg_parent(Self::IS_PARENT2_OCCUPIED_OFFSET, Self::PARENT2_OFFSET)
    }

    fn get_wild_pkm(&self) -> pkm::Pk7 {
        self.default_read::<pkm::Pk7Data>(Self::WILD_OFFSET).into()
    }

    fn get_is_egg_ready(&self) -> bool {
        self.default_read::<u8>(Self::EGG_READY_OFFSET) != 0
    }

    fn get_has_item(&self, offset: usize, item_id: u32, count: u32) -> bool {
        let item_info = self.default_read::<u32>(offset);
        let found_item_id = item_info & 0x3ff;
        let found_item_count = (item_info << 12) >> 22;

        found_item_id == item_id && found_item_count >= count
    }

    fn get_has_shiny_charm(&self) -> bool {
        self.get_has_item(Self::SHINY_CHARM_OFFSET, 632, 1)
    }
}
