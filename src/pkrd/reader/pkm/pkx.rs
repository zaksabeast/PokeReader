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
}
