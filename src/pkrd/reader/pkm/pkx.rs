use super::types;
use crate::pkrd::reader::Reader;

pub trait Pkx: Reader {
    fn encryption_constant(&self) -> u32 {
        self.read(0).unwrap()
    }

    fn species(&self) -> types::Species;

    fn pid(&self) -> u32;

    fn tid(&self) -> u16;

    fn sid(&self) -> u16;

    fn tsv(&self) -> u16 {
        (self.tid() ^ self.sid()) >> 4
    }

    fn nature(&self) -> types::Nature;

    fn ability(&self) -> types::Ability;

    fn ability_number(&self) -> u8;

    fn iv32(&self) -> u32;

    fn ivs(&self) -> types::Stats {
        let iv32 = self.iv32();
        types::Stats {
            hp: iv32 & 0x1F,
            atk: (iv32 >> 5) & 0x1F,
            def: (iv32 >> 10) & 0x1F,
            spe: (iv32 >> 15) & 0x1F,
            spa: (iv32 >> 20) & 0x1F,
            spd: (iv32 >> 25) & 0x1F,
        }
    }
}
