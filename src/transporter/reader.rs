use crate::pnp;
use pkm_rs::{Pk7, PokeCrypto};

struct TransporterAddresses {
    initial_seed: u32,
    mt_start: u32,
    mt_state_index: u32,
    transported_pokemon: u32,
}

const TRANSPORTER_ADDRESSES: TransporterAddresses = TransporterAddresses {
    initial_seed: 0x8afaf8c,
    mt_start: 0x8afaf94,
    mt_state_index: 0x8afaf90,
    transported_pokemon: 0x8bc6524,
};

pub struct TransporterReader {
    addrs: &'static TransporterAddresses,
}

impl TransporterReader {
    pub fn new() -> Self {
        Self {
            addrs: &TRANSPORTER_ADDRESSES,
        }
    }

    pub fn initial_seed(&self) -> u32 {
        pnp::read(self.addrs.initial_seed)
    }

    fn mt_state_index(&self) -> u32 {
        pnp::read(self.addrs.mt_state_index)
    }

    pub fn mt_state(&self) -> u32 {
        let index = self.mt_state_index();
        pnp::read(self.addrs.mt_start + if index != 624 { index * 4 } else { 0 })
    }

    fn read_pk7(&self, offset: u32) -> Pk7 {
        let bytes = pnp::read_array::<{ Pk7::STORED_SIZE }>(offset);
        Pk7::new_valid(bytes)
    }

    pub fn transported_pkm(&self, slot: u32) -> Pk7 {
        let offset = (slot * 0xe8) + self.addrs.transported_pokemon;
        self.read_pk7(offset)
    }
}
