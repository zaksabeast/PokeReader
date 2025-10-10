use super::reader::Gen7Reader;
use crate::{
    pnp,
    rng::{RngWrapper, Sfmt},
    utils::{format_egg_parent, is_daycare_masuda_method},
};

pub use crate::draw::{GREEN, PkxType, RED, WHITE, draw_header, draw_pkx, draw_pkx_brief, get_pp, print_pp};

pub fn draw_rng(reader: &Gen7Reader, rng: &RngWrapper<Sfmt>) {
    let sfmt_state = rng.current_state();

    pnp::println!("Seed:     {:08X}", rng.init_seed());
    pnp::println!("State[1]: {:08X}", (sfmt_state & 0xffffffff) as u32);
    pnp::println!("State[0]: {:08X}", (sfmt_state >> 32) as u32);
    pnp::println!("Advances: {}", rng.advances());
    pnp::println!("");
    pnp::println!("Gen7TID: {}", reader.g7tid());
    pnp::println!("TSV: {}, TRV: {:X}", reader.tsv(), reader.trv());
    pnp::println!("");
    pnp::println!("NPC count: {}", reader.npc_count());
}

pub fn draw_citra_info(reader: &Gen7Reader) {
    let main_rng_seed_context = reader.main_rng_seed_context();
    let datetime = main_rng_seed_context.game_start;
    pnp::println!("Seed ticks: {:08X}", main_rng_seed_context.ticks);
    pnp::println!("Seed date: {}", datetime.format("%b %d %Y"));
    pnp::println!("Seed time: {}", datetime.format("%H:%M:%S"));
    pnp::println!("Time offset: {}", main_rng_seed_context.time_offset_ms);
}

pub fn draw_sos(reader: &Gen7Reader, slot: u32, correction: u32) {
    pnp::println!("SOS Seed: {:08X}", reader.sos_seed());
    pnp::println!("SOS Chain Length: {}", reader.sos_chain());
    if reader.orb_active() {
        pnp::println!(color = GREEN, "Orb Active")
    } else {
        pnp::println!(color = RED, "Orb Not Active");
    }
    pnp::println!("Caller Slot: {}", slot);
    print_pp(get_pp(&reader.sos_caller_pkm(slot)));
    pnp::println!("");
    pnp::println!("Ally Data (Slot {}):", reader.ally_slot(slot, correction) + 1);
    draw_pkx_brief(&reader.sos_ally_pkm(slot, correction));
}

pub fn draw_daycare(reader: &Gen7Reader) {
    let is_egg_ready = reader.is_egg_ready();
    let parent1 = reader.egg_parent_1();
    let parent2 = reader.egg_parent_2();
    let egg_seed = reader.egg_seed();
    let has_shiny_charm = reader.has_shiny_charm();
    let is_masuda_method = is_daycare_masuda_method(&parent1, &parent2);

    pnp::println!(
        color = if is_egg_ready { GREEN } else { WHITE },
        "Egg Ready: {}",
        is_egg_ready
    );
    pnp::println!("{}", format_egg_parent(1, &parent1));
    pnp::println!("{}", format_egg_parent(2, &parent2));
    pnp::println!("");
    pnp::println!("Egg[3]: {:08X}", egg_seed[3]);
    pnp::println!("Egg[2]: {:08X}", egg_seed[2]);
    pnp::println!("Egg[1]: {:08X}", egg_seed[1]);
    pnp::println!("Egg[0]: {:08X}", egg_seed[0]);
    pnp::println!("");
    pnp::println!(
        color = if has_shiny_charm { GREEN } else { WHITE },
        "Shiny Charm: {}",
        has_shiny_charm
    );
    pnp::println!(
        color = if is_masuda_method { GREEN } else { WHITE },
        "Masuda Method: {}",
        is_masuda_method
    );
}
