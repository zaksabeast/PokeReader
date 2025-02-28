use crate::title::title_id;
use crate::title::SupportedTitle;
use crate::utils::game_fn;

game_fn!(xy_get_seed_hash() -> u64 = 0x10cad8);
game_fn!(oras_get_seed_hash() -> u64 = 0x10ca94);

pub fn get_seed_hash() -> u64 {
    match title_id() {
        SupportedTitle::X | SupportedTitle::Y => xy_get_seed_hash(),
        SupportedTitle::Or | SupportedTitle::As => oras_get_seed_hash(),
        _ => 0,
    }
}
