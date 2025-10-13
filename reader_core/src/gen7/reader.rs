use super::{game_lib, hook};
use crate::pnp;
use core::num::{NonZeroU8, NonZeroU32};
use pkm_rs::{Pk7, PokeCrypto};

struct Gen7Addresses {
    initial_seed: u32,
    sfmt_state_index: u32,
    sfmt_state: u32,
    _sos_base_addr: u32,
    sos_sfmt_state: u32,
    party: u32,
    wild: u32,
    sos_index: u32,
    sos_chain_length: u32,
    sos_battle_table: u32,
    pkm_container_base: u32,
    _prev_call_succeed: u32,
    orb_active: u32,
    // To be used in the future vvv
    _ally_id: u32,
    //
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
    npc_head_blinking_offset: u32,
    pk7_data_size: u32,
}

const SM_ADDRESSES: Gen7Addresses = Gen7Addresses {
    initial_seed: 0x325a3878,
    sfmt_state_index: 0x33196548,
    sfmt_state: 0x33195b88,
    _sos_base_addr: 0x30038C44,
    sos_sfmt_state: 0x30038C54,
    sos_battle_table: 0x30000420,
    pkm_container_base: 0x30004DA8,
    party: 0x34195e10,
    wild: 0x3002f7b8,
    sos_index: 0x30039614,
    sos_chain_length: 0x3003960d,
    _prev_call_succeed: 0x3003961f,
    orb_active: 0x3003961c,
    _ally_id: 0x3003961e,
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
    npc_list: 0x341977c4,
    npc_head_blinking_offset: 0x2f4,
    pk7_data_size: 0x330,
};

const USUM_ADDRESSES: Gen7Addresses = Gen7Addresses {
    initial_seed: 0x32663bf0,
    sfmt_state_index: 0x330d3f98,
    sfmt_state: 0x330d35d8,
    _sos_base_addr: 0x30038E20,
    sos_sfmt_state: 0x30038E30,
    sos_battle_table: 0x30000420,
    pkm_container_base: 0x30004DA8,
    party: 0x33f7fa44,
    wild: 0x3002f9a0,
    sos_index: 0x300397F0,
    sos_chain_length: 0x300397f9,
    _prev_call_succeed: 0x300397fb,
    orb_active: 0x300397f8,
    _ally_id: 0x300397fA,
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
    npc_head_blinking_offset: 0x2fc,
    pk7_data_size: 0x330,
};

pub struct Gen7Reader {
    is_usum: bool,
    addrs: &'static Gen7Addresses,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gen7WildSide {
    Left,
    Right,
    Invalid,
}

impl Gen7WildSide {
    pub fn new(value: usize) -> Self {
        match value {
            1 => Gen7WildSide::Right,
            2 => Gen7WildSide::Left,
            _ => Gen7WildSide::Invalid,
        }
    }
    pub fn other(&self) -> Self {
        match self {
            Gen7WildSide::Left => Gen7WildSide::Right,
            Gen7WildSide::Right => Gen7WildSide::Left,
            Gen7WildSide::Invalid => Gen7WildSide::Invalid,
        }
    }

    pub fn offset(&self) -> Option<u32> {
        match self {
            Gen7WildSide::Left => Some(0x4),
            Gen7WildSide::Right => Some(0x0),
            Gen7WildSide::Invalid => None,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Gen7WildSide::Left => "Left",
            Gen7WildSide::Right => "Right",
            Gen7WildSide::Invalid => "Invalid",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gen7PkmSlot {
    A,
    B,
    C,
    D,
    E,
    F,
    Invalid,
}

impl Gen7PkmSlot {
    pub fn new(value: usize) -> Self {
        match value {
            1 => Gen7PkmSlot::A,
            2 => Gen7PkmSlot::B,
            3 => Gen7PkmSlot::C,
            4 => Gen7PkmSlot::D,
            5 => Gen7PkmSlot::E,
            6 => Gen7PkmSlot::F,
            _ => Gen7PkmSlot::Invalid,
        }
    }
    pub fn offset(&self) -> Option<u32> {
        match self {
            Gen7PkmSlot::A => Some(0x0),
            Gen7PkmSlot::B => Some(0x1E4),
            Gen7PkmSlot::C => Some(0x3c8),
            Gen7PkmSlot::D => Some(0x5ac),
            Gen7PkmSlot::E => Some(0x790),
            Gen7PkmSlot::F => Some(0x974),
            Gen7PkmSlot::Invalid => None,
        }
    }
    pub fn label(&self) -> &'static str {
        match self {
            Gen7PkmSlot::A => "A",
            Gen7PkmSlot::B => "B",
            Gen7PkmSlot::C => "C",
            Gen7PkmSlot::D => "D",
            Gen7PkmSlot::E => "E",
            Gen7PkmSlot::F => "F",
            Gen7PkmSlot::Invalid => "Invalid",
        }
    }
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

    pub fn is_usum(&self) -> bool {
        self.is_usum
    }

    pub fn g7tid(&self) -> u32 {
        let sidtid = pnp::read::<u32>(self.addrs.id);

        sidtid % 1000000
    }

    fn tid(&self) -> u16 {
        pnp::read::<u16>(self.addrs.id)
    }

    fn sid(&self) -> u16 {
        pnp::read::<u16>(self.addrs.id + 2)
    }

    pub fn tsv(&self) -> u16 {
        (self.tid() ^ self.sid()) >> 4
    }

    pub fn trv(&self) -> u16 {
        (self.tid() ^ self.sid()) & 0xf
    }

    pub fn init_seed(&self) -> u32 {
        pnp::read(self.addrs.initial_seed)
    }

    fn sfmt_state_index(&self) -> u32 {
        let index = pnp::read(self.addrs.sfmt_state_index);
        if index != 624 { index } else { 0 }
    }

    pub fn sfmt_state(&self) -> u64 {
        pnp::read(self.addrs.sfmt_state + self.sfmt_state_index() * 4)
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

    // We still use this naive check to hint to the RNG wrapper when it can rest.
    pub fn sos_index(&self) -> u16 {
        let index = pnp::read(self.addrs.sos_index);
        if index != 624 { index } else { 0 }
    }

    pub fn sos_state(&self) -> u32 {
        pnp::read(self.addrs.sos_sfmt_state + (self.sos_index() as u32 * 4))
    }

    /* Note: Not very useful...
     * Value does not update until after last
     * input, which makes it rather misleading. */
    pub fn _sos_prevcall(&self) -> bool {
        pnp::read_bool(self.addrs._prev_call_succeed)
    }

    pub fn sos_chain(&self) -> u8 {
        pnp::read(self.addrs.sos_chain_length)
    }

    pub fn orb_active(&self) -> bool {
        pnp::read_bool(self.addrs.orb_active)
    }

    pub fn wild_slot_lookup(&self, side: Gen7WildSide) -> Gen7PkmSlot {
        match side.offset() {
            Some(offset) => {
                let battle_table = self.addrs.sos_battle_table + offset;
                let pkx_container_ptr = pnp::read::<u32>(battle_table).clamp(
                    self.addrs.pkm_container_base,
                    self.addrs.pkm_container_base + (self.addrs.pk7_data_size * 6),
                );
                Gen7PkmSlot::new(
                    (((pnp::read::<u32>(pkx_container_ptr) + 0x40 - self.addrs.wild)
                        / self.addrs.pk7_data_size)
                        + 1)
                    .clamp(1, 6) as usize,
                )
            }
            None => Gen7PkmSlot::Invalid,
        }
    }

    pub fn read_wild_slot(&self, slot: Gen7PkmSlot) -> Pk7 {
        match slot.offset() {
            Some(offset) => self.read_pk7(self.addrs.wild + offset),
            None => Pk7::default(),
        }
    }

    pub fn read_wild_side(&self, side: Gen7WildSide) -> Pk7 {
        self.read_wild_slot(self.wild_slot_lookup(side))
    }

    fn read_pk7(&self, offset: u32) -> Pk7 {
        let bytes = pnp::read_array::<{ Pk7::STORED_SIZE }>(offset);
        Pk7::new_valid(bytes)
    }

    pub fn party_pkm(&self, slot: Gen7PkmSlot) -> Pk7 {
        match slot.offset() {
            Some(offset) => self.read_pk7(self.addrs.party + offset),
            None => Pk7::default(),
        }
    }

    fn egg_parent(&self, is_present: u32, pkm: u32) -> Option<Pk7> {
        let is_parent_present = pnp::read_bool(is_present);

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

    pub fn box_pkm(&self) -> Pk7 {
        self.read_pk7(self.addrs.box_cursor)
    }

    pub fn pelago_pkm(&self, slot: u32) -> Pk7 {
        self.read_pk7((slot * 236) + self.addrs.pelago)
    }

    pub fn is_egg_ready(&self) -> bool {
        pnp::read_bool(self.addrs.egg_ready)
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
                let blink_type = NonZeroU32::new(pnp::read::<u32>(npc + self.addrs.npc_head_blinking_offset))
                    .and_then(|struct_ptr| NonZeroU32::new(pnp::read::<u32>(struct_ptr.get() + 0x114)))
                    .and_then(|struct_ptr| NonZeroU8::new(pnp::read::<u8>(struct_ptr.get() + 0xde)))
                    .map(|blink_setting| blink_setting.get())
                    .unwrap_or_default();
                let is_blinking = blink_type == 1 || blink_type == 2;
                if is_blinking {
                    npc_count += 1;
                }
            }
        }

        npc_count.saturating_sub(1)
    }

    pub fn main_rng_seed_context(&self) -> hook::RngSeedContext {
        hook::main_rng_seed_context()
    }
}
