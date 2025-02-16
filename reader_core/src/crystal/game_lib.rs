use crate::utils::game_fn;

game_fn!(read_gb_mem(gb_addr: u32) -> u8 = 0x1690b0);

pub mod gb_mem {
    use super::*;

    pub fn read_u8(addr: u32) -> u8 {
        read_gb_mem(addr)
    }

    pub fn read_u16(addr: u32) -> u16 {
        (read_u8(addr) as u16) << 8 | read_u8(addr + 1) as u16
    }

    pub fn read_u32(addr: u32) -> u32 {
        (read_u16(addr) as u32) << 16 | read_u16(addr + 2) as u32
    }
}
