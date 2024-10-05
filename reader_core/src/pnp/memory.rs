use super::bindings;
use alloc::{vec, vec::Vec};
use binrw::{io::Cursor, BinRead, BinWrite, BinWriterExt};
use core::mem;

pub fn read_array<const SIZE: usize>(addr: u32) -> [u8; SIZE] {
    let mut out = [0; SIZE];
    unsafe {
        bindings::host_read_mem(addr, SIZE as u32, out.as_mut_ptr() as u32);
    }
    out
}

pub fn read_vec(addr: u32, size: u32) -> Vec<u8> {
    let mut out = vec![0; size as usize];
    unsafe {
        bindings::host_read_mem(addr, size, out.as_mut_ptr() as u32);
    }
    out
}

pub fn read<T: BinRead + Default>(addr: u32) -> T
where
    for<'a> T::Args<'a>: Default,
{
    let mut reader = Cursor::new(read_vec(addr, mem::size_of::<T>() as u32));
    T::read_le(&mut reader).unwrap_or_default()
}

pub fn write_slice(addr: u32, buf: &[u8]) {
    unsafe {
        bindings::host_write_mem(addr, buf.len() as u32, buf.as_ptr() as u32);
    }
}

pub fn write<T: BinWrite>(addr: u32, buf: &T)
where
    for<'a> T::Args<'a>: Default,
{
    let mut writer = Cursor::new(Vec::new());
    let _ = writer.write_le(buf);
    write_slice(addr, &writer.into_inner())
}
