use crate::alloc::string::ToString;
use crate::{pnp, utils::menu::MenuOptionValue};
use pkm_rs::{Nature, Pkx, Shiny};

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

#[derive(PartialEq, Eq)]
enum Stat {
    Hp,
    Atk,
    Def,
    SpA,
    SpD,
    Spe,
}
use Stat::*;

struct NatureStat {
    increase: Stat,
    decrease: Stat,
}

impl From<(Stat, Stat)> for NatureStat {
    fn from((increase, decrease): (Stat, Stat)) -> Self {
        Self { increase, decrease }
    }
}

fn nature_stat(nature: Nature) -> NatureStat {
    match nature {
        Nature::Hardy => (Atk, Atk),
        Nature::Lonely => (Atk, Def),
        Nature::Brave => (Atk, Spe),
        Nature::Adamant => (Atk, SpA),
        Nature::Naughty => (Atk, SpD),
        Nature::Bold => (Def, Atk),
        Nature::Docile => (Def, Def),
        Nature::Relaxed => (Def, Spe),
        Nature::Impish => (Def, SpA),
        Nature::Lax => (Def, SpD),
        Nature::Timid => (Spe, Atk),
        Nature::Hasty => (Spe, Def),
        Nature::Serious => (Spe, Spe),
        Nature::Jolly => (Spe, SpA),
        Nature::Naive => (Spe, SpD),
        Nature::Modest => (SpA, Atk),
        Nature::Mild => (SpA, Def),
        Nature::Quiet => (SpA, Spe),
        Nature::Bashful => (SpA, SpA),
        Nature::Rash => (SpA, SpD),
        Nature::Calm => (SpD, Atk),
        Nature::Gentle => (SpD, Def),
        Nature::Sassy => (SpD, Spe),
        Nature::Careful => (SpD, SpA),
        Nature::Quirky => (SpD, SpD),
    }
    .into()
}

fn nature_stat_str(nature_stat: &NatureStat, stat: Stat) -> &'static str {
    if nature_stat.increase == stat && nature_stat.decrease == stat {
        return " ";
    }
    if nature_stat.increase == stat {
        return "+";
    }
    if nature_stat.decrease == stat {
        return "-";
    }

    " "
}

macro_rules! print_stat {
    ($iv:expr, $ev:expr, $stat:expr, $nature_stat:expr, $name:expr) => {
        pnp::println!(
            color = get_iv_color($iv),
            "{:<4}IV: {:>2}{} EV: {}",
            $name,
            $iv,
            nature_stat_str($nature_stat, $stat),
            $ev
        );
    };
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

    let ev_hp = pkx.ev_hp();
    let ev_atk = pkx.ev_atk();
    let ev_def = pkx.ev_def();
    let ev_spa = pkx.ev_spa();
    let ev_spd = pkx.ev_spd();
    let ev_spe = pkx.ev_spe();

    let nature = pkx.nature_t();
    let nature_stat = nature_stat(nature);

    pnp::println!("{} {}", nature, species);
    pnp::println!("Ability: ({}) {}", pkx.ability_number_t(), ability);
    pnp::println!("PID: {:08X}", pkx.pid());
    pnp::println!(color = shiny_color, "PSV: {:04}, {}", pkx.psv(), shiny_type);
    pnp::println!("Friendship: {}", pkx.ht_friendship());
    pnp::println!("");
    pnp::println!("HPower: {}", pkx.hidden_power_t());
    print_stat!(iv_hp, ev_hp, Hp, &nature_stat, "HP ");
    print_stat!(iv_atk, ev_atk, Atk, &nature_stat, "Atk ");
    print_stat!(iv_def, ev_def, Def, &nature_stat, "Def ");
    print_stat!(iv_spa, ev_spa, SpA, &nature_stat, "SpA ");
    print_stat!(iv_spd, ev_spd, SpD, &nature_stat, "SpD ");
    print_stat!(iv_spe, ev_spe, Spe, &nature_stat, "Spe ");
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
