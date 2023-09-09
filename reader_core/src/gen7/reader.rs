use super::{game_lib, hook};
use crate::pnp;
use pkm_rs::{Pk7, Pkx};

struct Gen7Addresses {
    initial_seed: u32,
    sfmt_state_index: u32,
    sfmt_state: u32,
    party: u32,
    wild: u32,
    sos: u32,
    sos_chain_length: u32,
    pelago: u32,
    egg_ready: u32,
    egg: u32,
    parent1: u32,
    parent2: u32,
    is_parent1_occupied: u32,
    is_parent2_occupied: u32,
    shiny_charm: u32,
    id: u32,
    box_cursor: u32,
    npc_list: u32,
}

const SM_ADDRESSES: Gen7Addresses = Gen7Addresses {
    initial_seed: 0x325a3878,
    sfmt_state_index: 0x33196548,
    sfmt_state: 0x33195b88,
    party: 0x34195e10,
    wild: 0x3002f7b8,
    sos: 0x3002f7b8,
    sos_chain_length: 0x3003960d,
    pelago: 0x331110ca,
    egg_ready: 0x3313edd8,
    egg: 0x3313eddc,
    parent1: 0x3313ec01,
    parent2: 0x3313ecea,
    is_parent1_occupied: 0x3313ec00,
    is_parent2_occupied: 0x3313ece9,
    shiny_charm: 0x330d5930,
    id: 0x330d67d0,
    box_cursor: 0x30000298,
    npc_list: 0x341977b8,
};

const USUM_ADDRESSES: Gen7Addresses = Gen7Addresses {
    initial_seed: 0x32663bf0,
    sfmt_state_index: 0x330d3f98,
    sfmt_state: 0x330d35d8,
    party: 0x33f7fa44,
    wild: 0x3002f9a0,
    sos: 0x3002f9a0,
    sos_chain_length: 0x300397f9,
    pelago: 0x3304d16a,
    egg_ready: 0x3307b1e8,
    egg: 0x3307b1ec,
    parent1: 0x3307b011,
    parent2: 0x3307b0fa,
    is_parent1_occupied: 0x3307b010,
    is_parent2_occupied: 0x3307b0f9,
    shiny_charm: 0x33011930,
    id: 0x33012818,
    box_cursor: 0x30000298,
    npc_list: 0x33f81438,
};

pub struct Gen7Reader {
    is_usum: bool,
    addrs: &'static Gen7Addresses,
}

impl Gen7Reader {
    pub fn sm() -> Self {
        Self {
            is_usum: false,
            addrs: &SM_ADDRESSES,
        }
    }

    pub fn usum() -> Self {
        Self {
            is_usum: true,
            addrs: &USUM_ADDRESSES,
        }
    }

    pub fn tid(&self) -> u32 {
        let sidtid = pnp::read::<u32>(self.addrs.id);

        sidtid % 1000000
    }

    pub fn tsv(&self) -> u16 {
        let tid = pnp::read::<u16>(self.addrs.id);
        let sid = pnp::read::<u16>(self.addrs.id + 2);

        (tid ^ sid) >> 4
    }

    pub fn init_seed(&self) -> u32 {
        pnp::read(self.addrs.initial_seed)
    }

    fn sfmt_state_index(&self) -> u32 {
        pnp::read(self.addrs.sfmt_state_index)
    }

    pub fn sfmt_state(&self) -> u64 {
        let index = self.sfmt_state_index();
        pnp::read(self.addrs.sfmt_state + if index != 624 { index * 4 } else { 0 })
    }

    pub fn egg_seed(&self) -> [u32; 4] {
        [
            pnp::read(self.addrs.egg),
            pnp::read(self.addrs.egg + 0x4),
            pnp::read(self.addrs.egg + 0x8),
            pnp::read(self.addrs.egg + 0xc),
        ]
    }

    pub fn sos_seed(&self) -> u32 {
        hook::sos_seed()
    }

    pub fn sos_chain(&self) -> u8 {
        pnp::read(self.addrs.sos_chain_length)
    }

    fn read_pk7(&self, offset: u32) -> Pk7 {
        let bytes = pnp::read_array::<{ Pk7::STORED_SIZE }>(offset);
        Pk7::new_or_default(bytes)
    }

    pub fn party_pkm(&self, slot: u32) -> Pk7 {
        let offset = (slot * 484) + self.addrs.party;
        self.read_pk7(offset)
    }

    fn egg_parent(&self, is_present: u32, pkm: u32) -> Option<Pk7> {
        let is_parent_present = pnp::read::<u8>(is_present) != 0;

        if !is_parent_present {
            return None;
        }

        let parent = self.read_pk7(pkm);
        Some(parent)
    }

    pub fn egg_parent_1(&self) -> Option<Pk7> {
        self.egg_parent(self.addrs.is_parent1_occupied, self.addrs.parent1)
    }

    pub fn egg_parent_2(&self) -> Option<Pk7> {
        self.egg_parent(self.addrs.is_parent2_occupied, self.addrs.parent2)
    }

    pub fn wild_pkm(&self) -> Pk7 {
        self.read_pk7(self.addrs.wild)
    }

    pub fn box_pkm(&self) -> Pk7 {
        self.read_pk7(self.addrs.box_cursor)
    }

    pub fn pelago_pkm(&self, slot: u32) -> Pk7 {
        self.read_pk7((slot * 236) + self.addrs.pelago)
    }

    pub fn sos_pkm(&self) -> Pk7 {
        self.read_pk7(self.addrs.sos)
    }

    pub fn is_egg_ready(&self) -> bool {
        pnp::read::<u8>(self.addrs.egg_ready) != 0
    }

    fn has_item(&self, offset: u32, item_id: u32, count: u32) -> bool {
        if self.is_usum {
            game_lib::usum_has_item(offset, item_id, count)
        } else {
            game_lib::sm_has_item(offset, item_id, count)
        }
    }

    pub fn has_shiny_charm(&self) -> bool {
        self.has_item(self.addrs.shiny_charm, 632, 1)
    }

    pub fn npc_count(&self) -> u8 {
        let mut npc_count: u8 = 0;
        for index in 0..35 {
            let npc = pnp::read::<u32>(self.addrs.npc_list + (index * 4));
            let is_present = pnp::read::<u32>(npc + 0xbc) != 0 && pnp::read::<u32>(npc + 0xc0) != 0;
            let can_blink = pnp::read::<u32>(npc + 0xe8) == 0;

            if is_present && can_blink {
                npc_count += 1;
            }
        }

        npc_count.saturating_sub(1)
    }

    pub fn main_rng_seed_ticks(&self) -> u32 {
        hook::main_rng_seed_ticks()
    }

    pub fn main_rng_ms_epoch(&self) -> (u32, u32) {
        hook::main_rng_ms_epoch()
    }
}
