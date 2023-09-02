use super::bindings;
use alloc::{vec, vec::Vec};
use core::mem;
use no_std_io::{EndianRead, EndianWrite, Reader, Writer};

/// Reads an array.
///
/// # Examples
/// ```
/// use pnp::read_array;
///
/// let data = read_array::<4>(0x8000000);
/// assert_eq!(data, [0x00, 0x00, 0x00, 0x00]);
/// ```
pub fn read_array<const SIZE: usize>(addr: u32) -> [u8; SIZE] {
    let mut out = [0; SIZE];
    unsafe {
        bindings::host_read_mem(addr, SIZE as u32, out.as_mut_ptr() as u32);
    }
    out
}

/// Reads a vector.
///
/// # Examples
/// ```
/// use pnp::read_vec;
///
/// let data = read_vec(0x8000000, 4);
/// assert_eq!(data, [0x00, 0x00, 0x00, 0x00]);
/// ```
pub fn read_vec(addr: u32, size: u32) -> Vec<u8> {
    let mut out = vec![0; size as usize];
    unsafe {
        bindings::host_read_mem(addr, size, out.as_mut_ptr() as u32);
    }
    out
}

/// Reads a value that implements EndianRead and Default.
///
/// # Examples
/// ```
/// use no_std_io::EndianRead;
/// use pnp::read;
///
/// #[derive(Debug, Default, EndianRead, PartialEq)]
/// struct MyData {
///   field1: u32,
///   field2: u32,
/// }
///
/// let data = read::<MyData>(0x8000000);
/// assert_eq!(data, MyData { field1: 0, field2: 0 });
/// ```
pub fn read<T: EndianRead + Default>(addr: u32) -> T {
    read_vec(addr, mem::size_of::<T>() as u32).default_read_le(0)
}

/// Reads a bool.
///
/// # Examples
/// ```
/// use pnp::read_bool;
///
/// let data = read_bool(0x8000000);
/// assert_eq!(data, false);
/// ```
pub fn read_bool(addr: u32) -> bool {
    read_array::<1>(addr)[0] != 0
}

/// Writes a slice.
///
/// # Examples
/// ```
/// use pnp::write_slice;
///
/// let data: [u8; 2] = [0x01, 0x02];
/// write_slice(0x8000000, &data);
/// ```
pub fn write_slice(addr: u32, buf: &[u8]) {
    unsafe {
        bindings::host_write_mem(addr, buf.len() as u32, buf.as_ptr() as u32);
    }
}

/// Writes a value that implements EndianWrite.
///
/// # Examples
/// ```
/// use no_std_io::EndianWrite;
/// use pnp::write;
///
/// #[derive(Debug, Default, EndianWrite)]
/// struct MyData {
///   field1: u32,
///   field2: u32,
/// }
///
/// let data = MyData {
///   field1: 0xaabbccdd,
///   field2: 0x11223344,
/// };
/// write(0x8000000, &data);
/// ```
pub fn write<T: EndianWrite>(addr: u32, buf: &T) {
    let mut out_buf = vec![0; buf.get_size()];
    out_buf.checked_write_le(0, buf);
    write_slice(addr, &out_buf)
}
