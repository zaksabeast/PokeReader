use super::reader::Gen7Reader;
use crate::{
    ctr,
    rng::{RngWrapper, Sfmt32, Sfmt64},
    utils::{format_egg_parent, is_daycare_masuda_method},
};

pub use crate::draw::{GREEN, PkxType, RED, WHITE, draw_header, draw_pkx, draw_pkx_brief, get_pp, print_pp};

pub fn draw_rng(reader: &Gen7Reader, rng: &RngWrapper<Sfmt64>) {
    let sfmt_state = rng.current_state();

    ctr::println!("Seed:     {:08X}", rng.init_seed());
    ctr::println!("State[1]: {:08X}", (sfmt_state & 0xffffffff) as u32);
    ctr::println!("State[0]: {:08X}", (sfmt_state >> 32) as u32);
    ctr::println!("Advances: {}", rng.advances());
    ctr::println!("");
    ctr::println!("Gen7TID: {}", reader.g7tid());
    ctr::println!("TSV: {}, TRV: {:X}", reader.tsv(), reader.trv());
    ctr::println!("");
    ctr::println!("NPC count: {}", reader.npc_count());
}

pub fn draw_citra_info(reader: &Gen7Reader) {
    let main_rng_seed_context = reader.main_rng_seed_context();
    let datetime = main_rng_seed_context.game_start;
    ctr::println!("Seed ticks: {:08X}", main_rng_seed_context.ticks);
    ctr::println!("Seed date: {}", datetime.format("%b %d %Y"));
    ctr::println!("Seed time: {}", datetime.format("%H:%M:%S"));
    ctr::println!("Time offset: {}", main_rng_seed_context.time_offset_ms);
    ctr::println!("");
    ctr::println!("On citra: {}", ctr::is_citra());
}

pub fn draw_sos(reader: &Gen7Reader, sos_rng: &mut RngWrapper<Sfmt32>, slot: u32, correction: u32) {
    let sos_seed = reader.sos_seed();
    let sos_state = reader.sos_state();
    let sos_chain_length = reader.sos_chain();

    sos_rng.reinit_if_needed(sos_seed);
    sos_rng.update_advances(sos_state);

    ctr::println!("SOS Seed: {:08X}", sos_rng.init_seed());
    ctr::println!("SOS Chain Length: {}", sos_chain_length);
    ctr::println!("SOS State: {:08X}", sos_state);
    ctr::println!("SOS Index: {}", sos_rng.advances());
    ctr::println!("");
    if reader.orb_active() {
        ctr::println!(color = GREEN, "Orb Active")
    } else {
        ctr::println!(color = RED, "Orb Not Active");
    }
    ctr::println!("Caller Slot: {}", slot);
    print_pp(get_pp(&reader.sos_caller_pkm(slot)));
    ctr::println!("");
    ctr::println!("Ally Data (Slot {}):", reader.ally_slot(slot, correction) + 1);
    draw_pkx_brief(&reader.sos_ally_pkm(slot, correction));
}

pub fn draw_daycare(reader: &Gen7Reader) {
    let is_egg_ready = reader.is_egg_ready();
    let parent1 = reader.egg_parent_1();
    let parent2 = reader.egg_parent_2();
    let egg_seed = reader.egg_seed();
    let has_shiny_charm = reader.has_shiny_charm();
    let is_masuda_method = is_daycare_masuda_method(&parent1, &parent2);

    ctr::println!(
        color = if is_egg_ready { GREEN } else { WHITE },
        "Egg Ready: {}",
        is_egg_ready
    );
    ctr::println!("{}", format_egg_parent(1, &parent1));
    ctr::println!("{}", format_egg_parent(2, &parent2));
    ctr::println!("");
    ctr::println!("Egg[3]: {:08X}", egg_seed[3]);
    ctr::println!("Egg[2]: {:08X}", egg_seed[2]);
    ctr::println!("Egg[1]: {:08X}", egg_seed[1]);
    ctr::println!("Egg[0]: {:08X}", egg_seed[0]);
    ctr::println!("");
    ctr::println!(
        color = if has_shiny_charm { GREEN } else { WHITE },
        "Shiny Charm: {}",
        has_shiny_charm
    );
    ctr::println!(
        color = if is_masuda_method { GREEN } else { WHITE },
        "Masuda Method: {}",
        is_masuda_method
    );
}
