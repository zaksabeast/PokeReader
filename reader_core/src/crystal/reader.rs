use super::game_lib::gb_mem;
use super::pk2::Pk2;
use crate::pnp;

struct Gen2Addresses {
    div_ptr: u32,
    pc_reg_ptr: u32,
    gb_rng_ptr: u32,
}

const CRYSTAL_ADDRESSES: Gen2Addresses = Gen2Addresses {
    div_ptr: 0x22f794,
    pc_reg_ptr: 0x22f5fc,
    gb_rng_ptr: 0xffe1,
};

pub struct Gen2Reader {
    addrs: &'static Gen2Addresses,
}

impl Gen2Reader {
    pub fn crystal() -> Self {
        Self {
            addrs: &CRYSTAL_ADDRESSES,
        }
    }

    pub fn div(&self) -> u8 {
        pnp::read(pnp::read::<u32>(self.addrs.div_ptr))
    }

    pub fn pc_reg(&self) -> u16 {
        pnp::read(self.addrs.pc_reg_ptr)
    }

    pub fn party(&self, slot: u8) -> Pk2 {
        let poke_addr = 0xDCDF + (slot as u32 * 0x30);
        let experience = gb_mem::read_u32(poke_addr + 0x8);
        let atkdef = gb_mem::read_u8(poke_addr + 0x15);
        let spespc = gb_mem::read_u8(poke_addr + 0x16);
        let spec_index = gb_mem::read_u8(poke_addr);
        Pk2::new(spec_index, atkdef, spespc, experience)
    }

    pub fn wild(&self) -> Pk2 {
        let spec_index = gb_mem::read_u8(0xD206);
        let atkdef = gb_mem::read_u8(0xD20C);
        let spespc = gb_mem::read_u8(0xD20D);
        Pk2::new(spec_index, atkdef, spespc, 0)
    }

    pub fn rng_state(&self) -> u16 {
        gb_mem::read_u16(self.addrs.gb_rng_ptr)
    }

    pub fn time_seconds(&self) -> u8 {
        gb_mem::read_u8(0xff98)
    }

    pub fn time_minutes(&self) -> u8 {
        gb_mem::read_u8(0xff96)
    }

    pub fn time_hours(&self) -> u8 {
        gb_mem::read_u8(0xff94)
    }

    pub fn time_day(&self) -> u8 {
        gb_mem::read_u8(0xd4cb) % 7
    }

    pub fn dst(&self) -> bool {
        gb_mem::read_u8(0xd4bc) != 0
    }

    pub fn play_seconds(&self) -> u8 {
        gb_mem::read_u8(0xD4C7)
    }

    pub fn play_minutes(&self) -> u8 {
        gb_mem::read_u8(0xD4C6)
    }

    pub fn play_hours(&self) -> u8 {
        gb_mem::read_u8(0xD4C5)
    }
}
