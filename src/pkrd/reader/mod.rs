mod gen_6;
pub use gen_6::*;

mod gen_7;
pub use gen_7::*;

use core::mem;
use ctr::{
    res::{CtrResult, GenericResultCode},
    safe_transmute::transmute_one_pedantic,
};
use safe_transmute::TriviallyTransmutable;

pub trait Reader {
    fn get_data(&self) -> &[u8];

    fn read<T: TriviallyTransmutable>(&self, offset: usize) -> CtrResult<T> {
        let data = self.get_data();
        let offset_end = offset + mem::size_of::<T>();

        if data.len() < offset_end {
            return Err(GenericResultCode::InvalidSize.into());
        }

        transmute_one_pedantic(&data[offset..offset_end])
    }
}
