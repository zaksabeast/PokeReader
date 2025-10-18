use super::bindings;
use alloc::{vec, vec::Vec};
use binrw::{io::Cursor, BinRead, BinWrite, BinWriterExt};
use core::{mem, ops::Add, u32};

pub fn pa_from_va_ptr(ptr: u32) -> u32 {
    unsafe { bindings::pa_from_va_ptr(ptr) }
}

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

#[derive(Debug, Clone, Copy)]
pub struct CtrPtr {
    ptr: u32,
    min: u32,
    max: u32,
}

impl CtrPtr {
    pub const fn new(ptr: u32, min: u32, max: u32) -> Self {
        Self { ptr, min, max }
    }

    pub fn new_ptr(&self, ptr: u32) -> CtrPtr {
        Self::new(ptr, self.min, self.max)
    }

    pub const fn new_g6(ptr: u32) -> Self {
        // Todo: set proper min/max for gen6
        Self::new(ptr, u32::MIN, u32::MAX)
    }

    pub const fn new_g7(ptr: u32) -> Self {
        // Todo: set proper min/max for gen7
        Self::new(ptr, u32::MIN, u32::MAX)
    }

    pub fn is_valid(&self, size: u32) -> bool {
        self.ptr >= self.min && self.ptr + size <= self.max
    }

    #[cfg(test)]
    pub fn read<T: BinRead + Default>(&self) -> T
    where
        for<'a> T::Args<'a>: Default,
    {
        T::default()
    }

    #[cfg(not(test))]
    pub fn read<T: BinRead + Default>(&self) -> T
    where
        for<'a> T::Args<'a>: Default,
    {
        let size = core::mem::size_of::<T>() as u32;
        if !self.is_valid(size) {
            return T::default();
        }
        read::<T>(self.ptr)
    }

    #[cfg(test)]
    pub fn read_array<const SIZE: usize>(&self) -> [u8; SIZE] {
        [0; SIZE]
    }

    #[cfg(not(test))]
    pub fn read_array<const SIZE: usize>(&self) -> [u8; SIZE] {
        if !self.is_valid(SIZE as u32) {
            return [0; SIZE];
        }
        read_array::<SIZE>(self.ptr)
    }

    pub fn next(&self) -> Self {
        let ptr = self.read::<u32>();
        self.new_ptr(ptr)
    }

    pub fn map<F: FnOnce(u32) -> u32>(&self, f: F) -> CtrPtr {
        self.new_ptr(f(self.ptr))
    }

    pub fn map_next<F: FnOnce(u32) -> u32>(&self, f: F) -> CtrPtr {
        self.map(f).next()
    }

    pub fn as_ptr(&self) -> u32 {
        self.ptr
    }
}

impl Add<u32> for CtrPtr {
    type Output = Self;

    fn add(self, other: u32) -> Self {
        self.new_ptr(self.ptr.wrapping_add(other))
    }
}

impl Add<CtrPtr> for u32 {
    type Output = CtrPtr;

    fn add(self, other: CtrPtr) -> CtrPtr {
        other.new_ptr(self.wrapping_add(other.ptr))
    }
}
