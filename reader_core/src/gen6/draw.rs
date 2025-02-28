use super::{
    reader::{Daycare, Gen6Reader},
    rng::Gen6Rng,
};
use crate::{
    pnp,
    utils::{format_egg_parent, is_daycare_masuda_method},
};

pub use crate::draw::{draw_header, draw_pkx};

pub fn draw_tinymt(rng: &Gen6Rng) {
    let tinymt = rng.tinymt();
    let tinymt_seed = tinymt.init_seed();
    let tinymt_state = tinymt.current_state();

    pnp::println!("TinyMT seed:");
    pnp::println!("[3]{:08X} [2]{:08X}", tinymt_seed[3], tinymt_seed[2]);
    pnp::println!("[1]{:08X} [0]{:08X}", tinymt_seed[1], tinymt_seed[0]);
    pnp::println!("State:");
    pnp::println!("[3]{:08X} [2]{:08X}", tinymt_state[3], tinymt_state[2]);
    pnp::println!("[1]{:08X} [0]{:08X}", tinymt_state[1], tinymt_state[0]);
    pnp::println!("Advances: {}", tinymt.advances());
}

pub fn draw_mt(rng: &Gen6Rng) {
    let mt = rng.mt();
    pnp::println!("Init seed: {:08X}", mt.init_seed());
    pnp::println!("Curr state: {:08X}", mt.current_state());
    pnp::println!("MT Advances: {}", mt.advances());
}

pub fn draw_rng(reader: &Gen6Reader, rng: &Gen6Rng) {
    draw_mt(rng);
    pnp::println!("");
    draw_tinymt(rng);
    pnp::println!("");
    pnp::println!("Save var: {:08X}", reader.seed_save_variable());
    pnp::println!("TSV: {}", reader.tsv());
}

pub fn draw_daycare(daycare: &Daycare) {
    let is_masuda_method = is_daycare_masuda_method(&daycare.parent1, &daycare.parent2);

    pnp::println!("Egg Ready: {}", daycare.is_egg_ready);
    pnp::println!("{}", format_egg_parent(1, &daycare.parent1));
    pnp::println!("{}", format_egg_parent(2, &daycare.parent2));
    pnp::println!("");
    pnp::println!("Egg[1]: {:08X}", daycare.egg_seed[0]);
    pnp::println!("Egg[0]: {:08X}", daycare.egg_seed[1]);
    pnp::println!("");
    pnp::println!("Masuda Method: {}", is_masuda_method);
}

pub fn draw_radar(reader: &Gen6Reader, rng: &Gen6Rng) {
    let chain_count = reader.radar_chain();
    pnp::println!("Chain count {}", chain_count);
    pnp::println!("");
    draw_tinymt(rng);
}

pub fn draw_dex_nav(reader: &Gen6Reader, rng: &Gen6Rng) {
    let step = reader.dex_nav_step();
    let chain = reader.dex_nav_chain();

    draw_mt(rng);
    pnp::println!("");
    draw_tinymt(rng);
    pnp::println!("");
    pnp::println!("Step {}", step);
    pnp::println!("Chain {}", chain);
}

pub fn draw_seed_rng(reader: &Gen6Reader, rng: &Gen6Rng) {
    let datetime = pnp::os_time();

    draw_mt(rng);
    pnp::println!("");
    pnp::println!("");
    pnp::println!("Save var: {:08X}", reader.seed_save_variable());
    pnp::println!("Date: {}", datetime.format("%b %d %Y"));
    pnp::println!("Time: {}", datetime.format("%H:%M:%S"));
}
