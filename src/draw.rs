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

pub fn draw_header<T: MenuOptionValue + Eq>(main_menu: T, current_view: T) {
    if current_view == main_menu {
        pnp::println!("-> Accept");
        pnp::println!("");
    } else {
        pnp::println!("<- Back - {}", T::get_label(current_view));
        pnp::println!("");
    }
}
