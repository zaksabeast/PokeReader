use crate::ctr;
use alloc::{vec, vec::Vec};
use binrw::{BinRead, BinWrite, BinWriterExt, io::Cursor};
use core::mem;

pub fn pa_from_va_ptr(ptr: u32) -> u32 {
    ctr::pa_from_va_ptr(ptr)
}

pub fn is_memory_mapped(addr: u32) -> bool {
    ctr::query_memory(addr).is_ok()
}

#[cfg(target_os = "horizon")]
fn read_into(addr: u32, out: &mut [u8]) {
    unsafe { core::ptr::copy_nonoverlapping(addr as *const u8, out.as_mut_ptr(), out.len()) }
}

// Host builds don't have game memory to read.
#[cfg(not(target_os = "horizon"))]
fn read_into(_addr: u32, _out: &mut [u8]) {}

pub fn read_array<const SIZE: usize>(addr: u32) -> [u8; SIZE] {
    let mut out = [0; SIZE];
    read_into(addr, &mut out);
    out
}

pub fn read_vec(addr: u32, size: u32) -> Vec<u8> {
    let mut out = vec![0; size as usize];
    read_into(addr, &mut out);
    out
}

pub fn read<T: BinRead + Default>(addr: u32) -> T
where
    for<'a> T::Args<'a>: Default,
{
    let mut reader = Cursor::new(read_vec(addr, mem::size_of::<T>() as u32));
    T::read_le(&mut reader).unwrap_or_default()
}

#[cfg(target_os = "horizon")]
pub fn write_slice(addr: u32, buf: &[u8]) {
    unsafe { core::ptr::copy_nonoverlapping(buf.as_ptr(), addr as *mut u8, buf.len()) }
}

// Host builds don't have game memory to write.
#[cfg(not(target_os = "horizon"))]
pub fn write_slice(_addr: u32, _buf: &[u8]) {}

pub fn write<T: BinWrite>(addr: u32, buf: &T)
where
    for<'a> T::Args<'a>: Default,
{
    let mut writer = Cursor::new(Vec::new());
    let _ = writer.write_le(buf);
    write_slice(addr, &writer.into_inner())
}
