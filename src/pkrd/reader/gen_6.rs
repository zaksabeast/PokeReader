use ctr::res::CtrResult;
use safe_transmute::TriviallyTransmutable;

pub trait Gen6Reader {
    const MT_STATE_INDEX_OFFSET: usize;

    fn read<T: TriviallyTransmutable>(&self, offset: usize) -> CtrResult<T>;

    fn get_mt_state_index(&self) -> CtrResult<usize> {
        self.read(Self::MT_STATE_INDEX_OFFSET)
    }
}
