use super::{
    lookup_call_rate,
    reader::{Gen7Reader, Gen7WildSide},
};
use crate::{
    pnp,
    rng::{RngWrapper, Sfmt32, Sfmt64},
    utils::{format_egg_parent, is_daycare_masuda_method},
};
use pkm_rs::Pkx;

pub use crate::draw::{
    GREEN, PkxType, RED, WHITE, draw_header, draw_invalid_pkx, draw_pkx, draw_pkx_brief, get_pp, print_pp,
};

pub fn draw_rng(reader: &Gen7Reader, rng: &RngWrapper<Sfmt64>) {
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

pub fn draw_sos(reader: &Gen7Reader, rng: &mut RngWrapper<Sfmt32>, menu_val: usize) -> bool {
    let sos_seed: u32 = reader.sos_seed();
    let sos_state: u32 = reader.sos_state();
    let sos_chain: u16 = reader.sos_chain() as u16;
    let sos_index: u16 = reader.sos_index();

    rng.reinit_if_needed(sos_seed);
    if sos_seed > 0 {
        if sos_index | sos_chain > 0 {
            rng.update_advances(sos_state);
        }
    }

    let caller_side = Gen7WildSide::new(menu_val);
    let ally_side = caller_side.other();
    let caller_pkm = &reader.read_wild_side(caller_side);
    let ally_pkm = &reader.read_wild_side(ally_side);

    if !caller_pkm.is_valid() {
        return draw_invalid_pkx();
    }

    pnp::println!("SOS Seed: {:08X}", rng.init_seed());
    pnp::println!("SOS Index: {}", rng.advances());
    pnp::println!("SOS Chain Length: {}", sos_chain);

    if reader.orb_active() {
        pnp::println!(color = GREEN, "Orb Active")
    } else {
        pnp::println!(color = RED, "Orb Not Active");
    }

    pnp::println!("");
    if caller_pkm.is_valid() {
        pnp::println!(
            "{} {} ({}):",
            caller_pkm.species_t(),
            &reader.wild_slot_lookup(caller_side).label(),
            caller_side.label()
        );
        let call_rate = lookup_call_rate(caller_pkm, reader.is_usum());
        pnp::println!(
            color = if call_rate == 0 { RED } else { WHITE },
            "Call Rate: {}",
            call_rate
        );
        print_pp(get_pp(caller_pkm));
    } else {
        return true;
    }

    pnp::println!("");
    if reader.sos_chain() > 0 {
        pnp::println!(
            "{} {} ({}):",
            ally_pkm.species_t(),
            reader.wild_slot_lookup(ally_side).label(),
            ally_side.label()
        );
        if ally_pkm.is_valid() {
            draw_pkx_brief(ally_pkm);
            return false;
        }
    }
    pnp::println!("No Ally to display.");
    return true;
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
