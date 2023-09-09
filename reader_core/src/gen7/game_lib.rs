use crate::utils::game_fn;

game_fn!(usum_has_item(ptr: u32, item: u32, count: u32) -> bool = 0x4c6984);
game_fn!(sm_has_item(ptr: u32, item: u32, count: u32) -> bool = 0x4a9cf4);
