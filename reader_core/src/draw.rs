use crate::alloc::string::ToString;
use crate::{pnp, utils::menu::MenuOptionValue};
use pkm_rs::{Pkx, Shiny};

pub fn draw_pkx(pkx: &impl Pkx) {
    let species = pkx.species_t().to_string();
    let ability = pkx.ability_t().to_string();

    let shiny_type = match pkx.shiny_type() {
        Some(Shiny::Star) => "Star",
        Some(Shiny::Square) => "Square",
        None => "None",
    };

    pnp::println!("Species: {}", species);
    pnp::println!("PID: {:08X}", pkx.pid());
    pnp::println!("PSV: {:04}", pkx.psv());
    pnp::println!("Shiny: {}", shiny_type);
    pnp::println!("Nature: {}", pkx.nature_t());
    pnp::println!("Ability: ({}) {}", pkx.ability_number_t(), ability);
    pnp::println!(
        "IVs: {}/{}/{}/{}/{}/{}",
        pkx.iv_hp(),
        pkx.iv_atk(),
        pkx.iv_def(),
        pkx.iv_spa(),
        pkx.iv_spd(),
        pkx.iv_spe()
    );
    pnp::println!("HPower: {}", pkx.hidden_power_t());
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
