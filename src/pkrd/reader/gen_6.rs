use super::Reader;
use ctr::res::CtrResult;

pub trait Gen6Reader: Reader {
    const MT_STATE_INDEX_OFFSET: usize;

    fn get_mt_state_index(&self) -> CtrResult<usize> {
        self.read(Self::MT_STATE_INDEX_OFFSET)
    }
}
