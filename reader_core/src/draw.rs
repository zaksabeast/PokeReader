use crate::alloc::string::ToString;
use crate::{pnp, utils::menu::MenuOptionValue};
use pkm_rs::{Pkx, Shiny};

const WHITE: u32 = 0xffffff;
const GREEN: u32 = 0x00cc00;
const RED: u32 = 0xff0000;

fn get_shiny_color(is_shiny: bool) -> u32 {
    match is_shiny {
        true => GREEN,
        false => WHITE,
    }
}

fn get_iv_color(iv: u8) -> u32 {
    match iv {
        30 | 31 => GREEN,
        0 | 1 => RED,
        _ => WHITE,
    }
}

pub fn draw_pkx(pkx: &impl Pkx) {
    let species = pkx.species_t().to_string();
    let ability = pkx.ability_t().to_string();

    let shiny_type = match pkx.shiny_type() {
        Some(Shiny::Star) => "Star",
        Some(Shiny::Square) => "Square",
        None => "Not Shiny",
    };
    let shiny_color = get_shiny_color(pkx.is_shiny());
    let iv_hp = pkx.iv_hp();
    let iv_atk = pkx.iv_atk();
    let iv_def = pkx.iv_def();
    let iv_spa = pkx.iv_spa();
    let iv_spd = pkx.iv_spd();
    let iv_spe = pkx.iv_spe();

    pnp::println!("Species: {}", species);
    pnp::println!("Nature: {}", pkx.nature_t());
    pnp::println!("Ability: ({}) {}", pkx.ability_number_t(), ability);
    pnp::println!("PID: {:08X}", pkx.pid());
    pnp::println!(color = shiny_color, "PSV: {:04}, {}", pkx.psv(), shiny_type);
    pnp::println!("");
    pnp::println!("HPower: {}", pkx.hidden_power_t());
    pnp::println!(color = get_iv_color(iv_hp), "HP  IV: {}", iv_hp);
    pnp::println!(color = get_iv_color(iv_atk), "Atk IV: {}", iv_atk);
    pnp::println!(color = get_iv_color(iv_def), "Def IV: {}", iv_def);
    pnp::println!(color = get_iv_color(iv_spa), "SpA IV: {}", iv_spa);
    pnp::println!(color = get_iv_color(iv_spd), "SpD IV: {}", iv_spd);
    pnp::println!(color = get_iv_color(iv_spe), "Spe IV: {}", iv_spe);
}

pub fn draw_header<T: MenuOptionValue + Eq>(main_menu: T, current_view: T, is_locked: bool) {
    if is_locked {
        pnp::println!("Unlock X+Y");
    } else if current_view == main_menu {
        pnp::println!("-> Accept / Lock X+Y");
    } else {
        pnp::println!("<- Back / Lock X+Y");
    }

    pnp::println!("");
}
