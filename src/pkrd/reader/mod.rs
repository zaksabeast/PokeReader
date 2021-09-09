pub mod pkm;

mod gen_6;
pub use gen_6::*;

mod gen_7;
pub use gen_7::*;

use core::mem;
use ctr::{
    log,
    res::{CtrResult, GenericResultCode},
    safe_transmute::transmute_one_pedantic,
};
use safe_transmute::TriviallyTransmutable;

/// An interface to safely read values from a struct.
/// The implementor must implement the `get_data` method.
pub trait Reader {
    /// Returns the data to be read from.
    fn get_data(&self) -> &[u8];

    /// Safely reads any `TriviallyTransmutable` type.
    /// Errors will be returned if the offset does not have enough data for the target type.
    fn read<T: TriviallyTransmutable>(&self, offset: usize) -> CtrResult<T> {
        let data = self.get_data();
        let offset_end = offset + mem::size_of::<T>();

        if data.len() < offset_end {
            return Err(GenericResultCode::InvalidSize.into());
        }

        transmute_one_pedantic(&data[offset..offset_end])
    }

    /// Same as `read`, but returns a default value if the read is invalid.
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
