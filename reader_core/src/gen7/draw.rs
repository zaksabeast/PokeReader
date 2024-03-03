use super::reader::Gen7Reader;
use crate::{
    pnp,
    rng::{RngWrapper, Sfmt},
    utils::{format_egg_parent, is_daycare_masuda_method},
};
use chrono::DateTime;

pub use crate::draw::{draw_header, draw_pkx};

pub fn draw_rng(reader: &Gen7Reader, rng: &RngWrapper<Sfmt>) {
    let sfmt_state = rng.current_state();

    pnp::println!("Seed:     {:08X}", rng.init_seed());
    pnp::println!("State[1]: {:08X}", (sfmt_state & 0xffffffff) as u32);
    pnp::println!("State[0]: {:08X}", (sfmt_state >> 32) as u32);
    pnp::println!("Advances: {}", rng.advances());
    pnp::println!("");
    pnp::println!("Gen7TID: {}", reader.tid());
    pnp::println!("TSV: {}", reader.tsv());
    pnp::println!("");
    pnp::println!("NPC count: {}", reader.npc_count());
}

pub fn draw_citra_info(reader: &Gen7Reader) {
    let main_rng_seed_context = reader.main_rng_seed_context();
    let date = main_rng_seed_context.init_datetime;
    pnp::println!("Seed ticks: {:08X}", main_rng_seed_context.ticks);
    pnp::println!("Init date: {}", date.format("%b %d %Y"));
    pnp::println!("Init time: {}", date.format("%H:%M:%S"));
}

pub fn draw_sos(reader: &Gen7Reader) {
    pnp::println!("SOS Seed: {:08X}", reader.sos_seed());
    pnp::println!("SOS Chain Length: {}", reader.sos_chain());
    pnp::println!("");
    draw_pkx(&reader.sos_pkm());
}

pub fn draw_daycare(reader: &Gen7Reader) {
    let is_egg_ready = reader.is_egg_ready();
    let parent1 = reader.egg_parent_1();
    let parent2 = reader.egg_parent_2();
    let egg_seed = reader.egg_seed();
    let has_shiny_charm = reader.has_shiny_charm();
    let is_masuda_method = is_daycare_masuda_method(&parent1, &parent2);

    pnp::println!("Egg Ready: {}", is_egg_ready);
    pnp::println!("{}", format_egg_parent(1, &parent1));
    pnp::println!("{}", format_egg_parent(2, &parent2));
    pnp::println!("");
    pnp::println!("Egg[3]: {:08X}", egg_seed[3]);
    pnp::println!("Egg[2]: {:08X}", egg_seed[2]);
    pnp::println!("Egg[1]: {:08X}", egg_seed[1]);
    pnp::println!("Egg[0]: {:08X}", egg_seed[0]);
    pnp::println!("");
    pnp::println!("Shiny Charm: {}", has_shiny_charm);
    pnp::println!("Masuda Method: {}", is_masuda_method);
}
