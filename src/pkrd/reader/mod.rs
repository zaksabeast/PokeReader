/// Tools to read Pokemon.
pub mod pkm;

/// Tools to read Gen 6 specific values.
mod gen_6;
pub use gen_6::*;

/// Tools to read Gen 7 specific values.
mod gen_7;
pub use gen_7::*;

use alloc::{vec, vec::Vec};
use core::mem;
use ctr::{
    log,
    res::{CtrResult, GenericResultCode},
    safe_transmute::transmute_one_pedantic,
};
use safe_transmute::TriviallyTransmutable;

/// An interface to safely read values from a struct.
pub trait Reader {
    /// Returns the data to be read from.
    fn get_data(&self) -> &[u8];

    /// Safely reads any [TriviallyTransmutable] type.
    /// Errors will be returned if the offset does not have enough data for the target type.
    ///
    /// All read data is copied, so anything returned from this can be manipualted without fear
    /// of corrupting the data source.
    fn read<T: TriviallyTransmutable>(&self, offset: usize) -> CtrResult<T> {
        let data = self.get_data();
        let result_size = mem::size_of::<T>();
        let offset_end = offset + result_size;

        if data.len() < offset_end {
            return Err(GenericResultCode::InvalidSize.into());
        }

        let mut copy: Vec<u8> = vec![0; result_size];
        copy.copy_from_slice(&data[offset..offset_end]);
        transmute_one_pedantic(&copy)
    }

    /// Same as [Reader::read], but returns a default value if the read is invalid.
    /// This will also log an error in debug builds.
    fn default_read<T: TriviallyTransmutable + Default>(&self, offset: usize) -> T {
        let result = self.read(offset);

        if let Err(result_code) = result {
            log(&alloc::format!(
                "Failed read in default_read: {:x}",
                result_code
            ))
        }

        result.unwrap_or_default()
    }
}
