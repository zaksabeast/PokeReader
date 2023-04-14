use pkm_rs::{Ek7, Pk7};

struct TransporterAddresses {
    initial_seed_patch: u32,
    initial_seed: u32,
    mt_start: u32,
    mt_state_index: u32,
    transported_pokemon: u32,
}

const TRANSPORTER_ADDRESSES: TransporterAddresses = TransporterAddresses {
    initial_seed_patch: 0x117fdc,
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
        pnp::read::<Ek7>(offset).into()
    }

    pub fn transported_pkm(&self, slot: u32) -> Pk7 {
        let offset = (slot * 0xe8) + self.addrs.transported_pokemon;
        self.read_pk7(offset)
    }

    pub fn patch_inital_seed_read(&self) {
        /*
         * The MT table initialization in gen 6 has a very useful nop instruction at the beginning of the function.
         * We overwrite this with str r1, [r0, #-4].
         * r1 is the register that contains the initial seed and r0 is the register that contains the memory address for the MT table.
         * The #-4 is to indicate write the initial seed 4 bytes before the MT table.
         * After this instruction is executed we can read the memory address 4 bytes before the MT table to get the initial seed.
         */
        pnp::write(self.addrs.initial_seed_patch, &0xe5001004u32);
    }
}
