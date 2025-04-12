use super::{
    game_lib::get_seed_hash,
    reader::{Daycare, Gen6Reader},
    rng::Gen6Rng,
};
use crate::{
    pnp,
    utils::{format_egg_parent, is_daycare_masuda_method},
};

pub use crate::draw::{draw_header, draw_pkx};

pub fn draw_tinymt(reader: &Gen6Reader, rng: &Gen6Rng) {
    let tinymt = rng.tinymt();
    let tinymt_state = tinymt.current_state();
    let tinymt_init_state = tinymt.rng().initial_state();
    let tinymt_seed = reader.tinymt_seed();

    pnp::println!("Tiny u32 seed: {:08X}", tinymt_seed);
    pnp::println!("Advances: {}", tinymt.advances());
    pnp::println!("");
    pnp::println!("TinyMT seed:");
    pnp::println!(
        "[3]{:08X} [2]{:08X}",
        tinymt_init_state[3],
        tinymt_init_state[2]
    );
    pnp::println!(
        "[1]{:08X} [0]{:08X}",
        tinymt_init_state[1],
        tinymt_init_state[0]
    );
    pnp::println!("TinyMT state:");
    pnp::println!("[3]{:08X} [2]{:08X}", tinymt_state[3], tinymt_state[2]);
    pnp::println!("[1]{:08X} [0]{:08X}", tinymt_state[1], tinymt_state[0]);
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
    draw_tinymt(reader, rng);
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
    draw_tinymt(reader, rng);
}

pub fn draw_dex_nav(reader: &Gen6Reader, rng: &Gen6Rng) {
    let step = reader.dex_nav_step();
    let chain = reader.dex_nav_chain();

    draw_mt(rng);
    pnp::println!("");
    draw_tinymt(reader, rng);
    pnp::println!("");
    pnp::println!("Step {}", step);
    pnp::println!("Chain {}", chain);
}

pub fn draw_seed_rng(reader: &Gen6Reader, rng: &Gen6Rng) {
    let datetime = pnp::os_time();
    let hash = get_seed_hash();
    let seed_hash = (hash as u32) ^ (hash >> 32) as u32;

    draw_mt(rng);
    pnp::println!("");
    pnp::println!("");
    pnp::println!("Save var: {:08X}", reader.seed_save_variable());
    pnp::println!("Date: {}", datetime.format("%b %d %Y"));
    pnp::println!("Time: {}", datetime.format("%H:%M:%S"));
    pnp::println!("");
    pnp::println!("Seed hash: {:08X}", seed_hash);
}

const MIRAGE_SPOT_NAMES: [&str; 34] = [
    "None",
    "Crescent Isle",
    "East of Mossdeep",
    "North of Route 124",
    "West of Route 114",
    "North of Lilycove",
    "South of Route 132",
    "West of Route 105",
    "South of Route 109",
    "North of Route 111",
    "West of Rustboro",
    "North of Fortree",
    "South of Pacifidlog",
    "South of Route 107",
    "North of Route 124",
    "North of Route 132",
    "Southeast of Route 129",
    "North of Fallarbor",
    "West of Route 104",
    "South of Route 134",
    "North of Route 124",
    "West of Dewford Town",
    "South of Pacifidlog",
    "South of Route 132",
    "North of Route 113",
    "East of Shoal Cave",
    "West of Route 104",
    "North of Lilycove",
    "Northeast of Route 125",
    "West of Route 131",
    "North of Mossdeep",
    "South of Route 129",
    "Southeast of Route 129",
    "East of Mossdeep",
];

const MIRAGE_POKEMON: [&[&str]; 34] = [
    &["None"],
    &["Cresselia"],
    &["Tangela", "Sunkern", "Glameow", "Minccino"],
    &["Tangela", "Sunkern", "Purugly", "Vulpix"],
    &["Tangela", "Sunkern", "Purugly", "Petilil"],
    &["Tangela", "Sunkern", "Purugly", "Cherrim"],
    &["Sunkern", "Petilil", "Audino"],
    &["Forretress", "Happiny"],
    &["Audino", "Sunkern"],
    &["Kricketune", "Larvesta"],
    &["Tynamo", "Klink", "Boldore", "Graveler"],
    &["Klink", "Tynamo", "Excadrill", "Onix"],
    &["Tynamo", "Cofagrigus", "Slowpoke"],
    &["Unown"],
    &["Klink", "Cofagrigus", "Graveler", "Boldore"],
    &["Ditto", "Excadrill", "Tynamo"],
    &["Tynamo", "Onix", "Graveler", "Boldore"],
    &["Slowpoke", "Tynamo"],
    &["Venomoth", "Xatu", "Zebstrika", "Darmanitan"],
    &["Venomoth", "Xatu", "Zebstrika", "Maractus"],
    &["Venomoth", "Xatu", "Zebstrika", "Persian"],
    &["Venomoth", "Xatu", "Zebstrika", "Tangela"],
    &["Audino", "Xatu"],
    &["Munna", "Ditto"],
    &["Darmanitan", "Larvesta"],
    &["Purugly", "Porygon"],
    &["Forretress", "Donphan", "Kricketune", "Stantler"],
    &["Forretress", "Donphan", "Kricketune", "Rufflet"],
    &["Forretress", "Donphan", "Kricketune", "Vullaby"],
    &["Donphan", "Kricketune", "Girafarig"],
    &["Magby", "Darmanitan"],
    &["Zebstrika", "Elekid"],
    &["Porygon", "Xatu", "Munna"],
    &["Audino", "Happiny", "Tangela"],
];

pub fn draw_mirage_spot(reader: &Gen6Reader) {
    let id = reader.mirage_spot_id() as usize;
    let current_date = pnp::os_time().format("%b %d %Y");
    let last_save_date = reader.last_save_date().format("%b %d %Y");
    let mirage_spot_seed = reader.mirage_spot_seed();
    let mirage_pokemon = MIRAGE_POKEMON[id];

    pnp::println!("Mirage spot {}", id);
    pnp::println!("{}", MIRAGE_SPOT_NAMES[id]);
    for pokemon in mirage_pokemon.iter() {
        pnp::println!("- {}", pokemon);
    }
    pnp::println!("");
    pnp::println!("Seed: {:08x}", mirage_spot_seed);
    pnp::println!("Tid: {}", reader.tid());
    pnp::println!("Save date: {}", last_save_date);
    pnp::println!("Curr Date: {}", current_date);
    pnp::println!("");
    pnp::println!("Penalty min: {}", reader.time_penalty_hours());
}
