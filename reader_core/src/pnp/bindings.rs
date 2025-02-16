extern "C" {
    pub fn host_print(ptr: u32, size: u32);
    pub fn host_read_mem(game_addr: u32, size: u32, out_ptr: u32);
    pub fn host_write_mem(game_addr: u32, size: u32, in_ptr: u32);
    pub fn host_is_just_pressed(io_bits: u32) -> u32;
    pub fn host_set_print_max_len(max_len: u32);
    pub fn host_get_game_title_id() -> u64;
    pub fn host_game_start_ms() -> u64;
    pub fn get_current_keys() -> u32;
}

#[cfg(feature = "test_stubs")]
pub mod test_stubs {
    #[no_mangle]
    pub extern "C" fn host_print(_ptr: u32, _size: u32) {}
    #[no_mangle]
    pub extern "C" fn host_read_mem(_game_addr: u32, _size: u32, _out_ptr: u32) {}
    #[no_mangle]
    pub extern "C" fn host_write_mem(_game_addr: u32, _size: u32, _in_ptr: u32) {}
    #[no_mangle]
    pub extern "C" fn host_is_just_pressed(_io_bits: u32) -> u32 {
        0
    }
    #[no_mangle]
    pub extern "C" fn host_set_print_max_len(_max_len: u32) {}
    #[no_mangle]
    pub extern "C" fn host_get_game_title_id() -> u64 {
        0
    }
    #[no_mangle]
    pub extern "C" fn host_game_start_ms() -> u64 {
        0
    }
    #[no_mangle]
    pub extern "C" fn get_current_keys() -> u32 {
        0
    }
}
