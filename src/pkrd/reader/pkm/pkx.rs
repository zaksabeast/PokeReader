use super::types;
use crate::pkrd::reader::Reader;

pub trait Pkx: Reader {
    fn encryption_constant(&self) -> u32 {
        self.read(0).unwrap()
    }

    fn species(&self) -> types::Species;
}
