use crate::title::{loaded_title, LoadedTitle};
use crate::utils::game_fn;

game_fn!(xy_get_seed_hash() -> u64 = 0x10cad8);
game_fn!(oras_get_seed_hash() -> u64 = 0x10ca94);

pub fn get_seed_hash() -> u64 {
    match loaded_title() {
        Ok(LoadedTitle::X) | Ok(LoadedTitle::Y) => xy_get_seed_hash(),
        Ok(LoadedTitle::Or) | Ok(LoadedTitle::As) => oras_get_seed_hash(),
        _ => 0,
    }
}
