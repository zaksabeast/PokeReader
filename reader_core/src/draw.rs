use super::title::{LoadedTitle, loaded_title};
use crate::alloc::string::ToString;
use crate::crystal::CRYSTAL_CYAN;
use crate::{GIT_HASH, VERSION};
use crate::{pnp, utils::menu::MenuOptionValue};
use pkm_rs::{Nature, Pkx, Shiny};

pub const WHITE: u32 = 0xffffff;
pub const RED: u32 = 0xff0000;
pub const MAGMA_RED: u32 = 0xFF4433;
pub const ORANGE: u32 = 0xd75f00;
pub const ULTRA_ORANGE: u32 = 0xff5f1f;
pub const YELLOW: u32 = 0xd7ff00;
pub const GREEN: u32 = 0x00cc00;
pub const CYAN: u32 = 0x00ffff;
pub const MUTED_CYAN: u32 = 0x00cccc;
pub const BLUE: u32 = 0x0000ff;
pub const PURPLE: u32 = 0xaf5fff;
pub const ULTRA_PURPLE: u32 = 0xaf00ff;
pub const HOT_PINK: u32 = 0xd7005f;

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

#[derive(PartialEq, Eq)]
pub enum PkxType {
    Wild,
    Tame,
}

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

pub fn get_pp(pkx: &impl Pkx) -> u32 {
    pkx.move1_pp() as u32 + pkx.move2_pp() as u32 + pkx.move3_pp() as u32 + pkx.move4_pp() as u32
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

pub fn print_pp(pp: u32) {
    let color = match pp >> 1 {
        1 => RED,
        2 => MAGMA_RED,
        3 => ORANGE,
        4 => ULTRA_ORANGE,
        5 => YELLOW,
        _ => WHITE,
    };
    pnp::println!(color = color, "PP Remaining: {}", pp);
}

pub fn print_title() {
    match loaded_title() {
        Ok(title) => match title {
            LoadedTitle::S => pnp::println!(color = ORANGE, " Pokemon Sun"),
            LoadedTitle::M => pnp::println!(color = PURPLE, " Pokemon Moon"),
            LoadedTitle::Us => pnp::println!(color = ULTRA_ORANGE, " Pokemon Ultra Sun"),
            LoadedTitle::Um => pnp::println!(color = ULTRA_PURPLE, " Pokemon Ultra Moon"),
            LoadedTitle::X => pnp::println!(color = CYAN, " Pokemon X"),
            LoadedTitle::Y => pnp::println!(color = HOT_PINK, " Pokemon Y"),
            LoadedTitle::Or => pnp::println!(color = MAGMA_RED, " Pokemon Omega Ruby"),
            LoadedTitle::As => pnp::println!(color = BLUE, " Pokemon Alpha Sapphire"),
            LoadedTitle::Transporter => pnp::println!(color = YELLOW, " Pokemon Transporter"),
            LoadedTitle::CrystalEn
            | LoadedTitle::CrystalDe
            | LoadedTitle::CrystalFr
            | LoadedTitle::CrystalEs
            | LoadedTitle::CrystalIt => {
                pnp::println!(color = 0xaf00d7, " Pokemon Crystal")
            }
        },
        Err(_error) => {
            pnp::println!(color = RED, "???")
        }
    }
}

pub fn shiny_type(pkx: &impl Pkx) -> &'static str {
    match pkx.shiny_type() {
        Some(Shiny::Star) => "Star",
        Some(Shiny::Square) => "Square",
        None => "Not Shiny",
    }
}

pub fn draw_pkx_brief(pkx: &impl Pkx) {
    let species = pkx.species_t().to_string();
    let ability = pkx.ability_t().to_string();

    let shiny_type = shiny_type(pkx);
    let shiny_color = get_shiny_color(pkx.is_shiny());
    let iv_hp = pkx.iv_hp();
    let iv_atk = pkx.iv_atk();
    let iv_def = pkx.iv_def();
    let iv_spa = pkx.iv_spa();
    let iv_spd = pkx.iv_spd();
    let iv_spe = pkx.iv_spe();

    let nature = pkx.nature_t();

    pnp::println!("{} {}", nature, species);
    pnp::println!("Ability: ({}) {}", pkx.ability_number_t(), ability);
    pnp::println!("PID: {:08X}", pkx.pid());
    pnp::println!(color = shiny_color, "PSV: {:04}, {}", pkx.psv(), shiny_type);
    pnp::println!("HPower: {}", pkx.hidden_power_t());
    pnp::println!(
        "IVs: {}/{}/{}/{}/{}/{}",
        iv_hp,
        iv_atk,
        iv_def,
        iv_spa,
        iv_spd,
        iv_spe
    );
}

pub fn draw_pkx(pkx: &impl Pkx, pkx_type: PkxType) {
    let species = pkx.species_t().to_string();
    let ability = pkx.ability_t().to_string();

    let shiny_type = shiny_type(pkx);
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
    if pkx_type == PkxType::Tame {
        // Friendship will always be zero for wild pokemon and does not fit
        pnp::println!("Friendship: {}", pkx.current_friendship());
    }
    pnp::println!("");
    pnp::println!("HPower: {}", pkx.hidden_power_t());
    if pkx_type == PkxType::Wild {
        // PP does not matter for Party/Box view as you can just summary
        print_pp(get_pp(pkx));
    }
    print_stat!(iv_hp, ev_hp, Hp, &nature_stat, "HP ");
    print_stat!(iv_atk, ev_atk, Atk, &nature_stat, "Atk ");
    print_stat!(iv_def, ev_def, Def, &nature_stat, "Def ");
    print_stat!(iv_spa, ev_spa, SpA, &nature_stat, "SpA ");
    print_stat!(iv_spd, ev_spd, SpD, &nature_stat, "SpD ");
    print_stat!(iv_spe, ev_spe, Spe, &nature_stat, "Spe ");
}

pub fn draw_controls_help() {
    pnp::println!("[Start] + [Select]:");
    pnp::println!(" - Pause Game");
    pnp::println!("");
    pnp::println!("[Start]/[A]:");
    pnp::println!(" - Unpause Game");
    pnp::println!("");
    pnp::println!("[Select]:");
    pnp::println!(" - Frame Advance");
}

pub fn draw_specific_help(draw_func: fn() -> ()) {
    draw_func();
}

pub fn draw_misc_help() {
    pnp::println!("PokeReader");
    draw_version();
    pnp::println!("");
    pnp::println!("Current Title:");
    print_title();
    pnp::println!("");
    pnp::println!("PokemonRNG Resources:");
    pnp::println!(color = MUTED_CYAN, " PokemonRNG.com");
    match loaded_title() {
        Ok(LoadedTitle::CrystalEn)
        | Ok(LoadedTitle::CrystalEs)
        | Ok(LoadedTitle::CrystalDe)
        | Ok(LoadedTitle::CrystalFr)
        | Ok(LoadedTitle::CrystalIt) => pnp::println!(color = CRYSTAL_CYAN, " discord.gg/d8JuAvg"),
        _ => pnp::println!(color = MUTED_CYAN, " discord.gg/d8JuAvg"),
    }
}

pub fn draw_version() {
    pnp::println!(" Ver {} {}", VERSION, GIT_HASH);
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
