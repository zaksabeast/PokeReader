use super::{pkm, Reader};
use ctr::res::CtrResult;

pub trait Gen6Reader: Reader {
    const INITIAL_SEED_OFFSET: usize;
    const MT_STATE_INDEX_OFFSET: usize;
    const PARTY_OFFSET: usize;
    const EGG_READY_OFFSET: usize;
    const EGG_OFFSET: usize;
    const PARENT1_OFFSET: usize;
    const PARENT2_OFFSET: usize;

    fn get_initial_seed(&self) -> CtrResult<usize> {
        self.read(Self::INITIAL_SEED_OFFSET)
    }

    fn get_mt_state_index(&self) -> CtrResult<usize> {
        self.read(Self::MT_STATE_INDEX_OFFSET)
    }

    fn get_party_pkm(&self, slot: pkm::PartySlot) -> pkm::Pk6 {
        let offset = ((slot as usize) * 484) + Self::PARTY_OFFSET;
        self.default_read::<pkm::Pk6Data>(offset).into()
    }
}
