use core::mem;
use ctr::{
    res::{CtrResult, GenericResultCode},
    safe_transmute::transmute_one_pedantic,
};
use safe_transmute::TriviallyTransmutable;

pub struct Reader<'a> {
    data: &'a [u8],
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    pub fn read<T: TriviallyTransmutable>(&self, offset: usize) -> CtrResult<T> {
        let offset_end = offset + mem::size_of::<T>();

        if self.data.len() < offset_end {
            return Err(GenericResultCode::InvalidSize.into());
        }

        transmute_one_pedantic(&self.data[offset..offset_end])
    }
}
