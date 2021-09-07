use ctr::res::CtrResult;
use safe_transmute::TriviallyTransmutable;

pub trait Gen7Reader {
    const INITIAL_SEED_OFFSET: usize;

    fn read<T: TriviallyTransmutable>(&self, offset: usize) -> CtrResult<T>;

    fn get_initial_seed(&self) -> CtrResult<u32> {
        self.read(Self::INITIAL_SEED_OFFSET)
    }
}
