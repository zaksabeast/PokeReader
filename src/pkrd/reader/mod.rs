/// Tools to read Pokemon.
pub mod pkm;

/// Tools to read Gen 6 specific values.
mod gen6;
pub use gen6::*;

/// Tools to read Gen 7 specific values.
mod gen7;
pub use gen7::*;

use crate::log;
use alloc::{vec, vec::Vec};
use core::{convert::TryInto, mem};
use ctr::{
    res::{CtrResult, GenericResultCode},
    safe_transmute::transmute_one_pedantic,
};
use safe_transmute::TriviallyTransmutable;

/// An interface to safely read values from a struct.
pub trait Reader {
    /// Returns the data to be read from.
    fn get_data(&self) -> &[u8];

    fn read_bytes<T: Sized>(&self, offset: usize) -> CtrResult<&[u8]> {
        let data = self.get_data();
        let result_size = mem::size_of::<T>();
        let offset_end = offset + result_size;

        if data.len() < offset_end {
            return Err(GenericResultCode::InvalidSize.into());
        }

        Ok(&data[offset..offset_end])
    }

    /// Safely reads any [TriviallyTransmutable] type.
    /// Errors will be returned if the offset does not have enough data for the target type.
    ///
    /// All read data is copied, so anything returned from this can be manipualted without fear
    /// of corrupting the data source.
    fn read<T: TriviallyTransmutable>(&self, offset: usize) -> CtrResult<T> {
        let bytes = self.read_bytes::<T>(offset)?;

        let mut copy: Vec<u8> = vec![0; bytes.len()];
        copy.copy_from_slice(bytes);
        transmute_one_pedantic(&copy)
    }

    /// Same as [Reader::read], but returns a default value if the read is invalid.
    /// This will also log an error in debug builds.
    fn default_read<T: TriviallyTransmutable + Default>(&self, offset: usize) -> T {
        let result = self.read(offset);

        if let Err(result_code) = result {
            log::error(&alloc::format!(
                "Failed read in default_read: {:x}",
                result_code
            ))
        }

        result.unwrap_or_default()
    }

    /// Same as [Reader::read], except the value is read from its little endian representation.
    /// Prefer [Reader::read] and [Reader::default_read] when possible.
    /// This should only be used when reading data from a format or protocol
    /// that explicitly defines little endian.
    fn read_le<T: EndianRead>(&self, offset: usize) -> CtrResult<T> {
        let bytes = self.read_bytes::<T>(offset)?;
        Ok(T::read_le(bytes))
    }

    /// /// Same as [Reader::default_read], except the value is read from its little endian representation.
    /// Prefer [Reader::read] and [Reader::default_read] when possible.
    /// This should only be used when reading data from a format or protocol
    /// that explicitly defines little endian.
    fn default_read_le<T: EndianRead + Default>(&self, offset: usize) -> T {
        let result = self.read_le(offset);

        if let Err(result_code) = result {
            log::error(&alloc::format!(
                "Failed read in default_read_le: {:x}",
                result_code
            ))
        }

        result.unwrap_or_default()
    }
}

pub trait EndianRead: Sized {
    fn read_le(bytes: &[u8]) -> Self;
}

impl EndianRead for u16 {
    fn read_le(bytes: &[u8]) -> Self {
        u16::from_le_bytes(bytes.try_into().unwrap())
    }
}

impl EndianRead for u32 {
    fn read_le(bytes: &[u8]) -> Self {
        u32::from_le_bytes(bytes.try_into().unwrap())
    }
}
