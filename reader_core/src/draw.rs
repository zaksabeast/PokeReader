use crate::alloc::string::ToString;
use crate::{pnp, utils::menu::MenuOptionValue};
use pkm_rs::{types::Shiny, Pkx};

pub fn draw_pkx(pkx: &impl Pkx) {
    let species = pkx.species().to_string();
    let ability = pkx.ability().to_string();

    let shiny_type = match pkx.shiny_type() {
        Some(Shiny::Star) => "Star",
        Some(Shiny::Square) => "Square",
        None => "None",
    };

    pnp::println!("Species: {}", species);
    pnp::println!("PID: {:08X}", pkx.pid());
    pnp::println!("PSV: {:04}", pkx.psv());
    pnp::println!("Shiny: {}", shiny_type);
    pnp::println!("Nature: {}", pkx.nature());
    pnp::println!("Ability: ({}) {}", pkx.ability_number(), ability);
    pnp::println!("IVs: {}", pkx.ivs());
    pnp::println!("HPower: {}", pkx.hidden_power());
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
