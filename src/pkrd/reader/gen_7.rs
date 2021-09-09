use super::Reader;
use ctr::res::CtrResult;

pub trait Gen7Reader: Reader {
    const INITIAL_SEED_OFFSET: usize;

    fn get_initial_seed(&self) -> CtrResult<u32> {
        self.read(Self::INITIAL_SEED_OFFSET)
    }
}
