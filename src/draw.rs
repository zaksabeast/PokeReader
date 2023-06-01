use crate::menu::MenuOptionValue;
use pkm_rs::Pkx;

pub fn draw_pkx(pkx: &impl Pkx) {
    let species = pkx.species().to_string();
    let ability = pkx.ability().to_string();

    pnp::println!("Species: {}", species);
    pnp::println!("PID: {:08X}", pkx.pid());
    pnp::println!("PSV: {:04}", pkx.psv());
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
