use crate::title::{supported_title, SupportedTitle};
use crate::utils::game_fn;

game_fn!(xy_get_seed_hash() -> u64 = 0x10cad8);
game_fn!(oras_get_seed_hash() -> u64 = 0x10ca94);

pub fn get_seed_hash() -> u64 {
    match supported_title() {
        SupportedTitle::X | SupportedTitle::Y => xy_get_seed_hash(),
        SupportedTitle::Or | SupportedTitle::As => oras_get_seed_hash(),
        _ => 0,
    }
}
