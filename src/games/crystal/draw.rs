use super::hook::{add_div_tracker, measured_div, rng_advance, sub_div_tracker};
use super::pk2::Pk2;
use super::reader::Gen2Reader;
use crate::ctr;
use crate::ctr::hid::{self, Button};
pub use crate::draw::draw_header;

const WHITE: u32 = 0xffffff;
const GREEN: u32 = 0x003c00;
const RED: u32 = 0x1f0000;

fn get_iv_color(iv: u8) -> u32 {
    match iv {
        15 => GREEN,
        0 => RED,
        _ => WHITE,
    }
}

fn get_shiny_color(is_shiny: bool) -> u32 {
    match is_shiny {
        true => GREEN,
        false => WHITE,
    }
}

pub fn draw_rng(reader: &Gen2Reader) {
    match add_div_tracker().index() {
        Some(index) => ctr::println!("ADIV Index {}", index),
        None => ctr::println!("Finding ADIV Index..."),
    };
    match sub_div_tracker().index() {
        Some(index) => ctr::println!("SDIV Index {}", index),
        None => ctr::println!("Finding SDIV Index..."),
    }
    ctr::println!("DIV {:04X}", measured_div());
    ctr::println!("State {:04X}", reader.rng_state());
    ctr::println!("Advances {}", rng_advance());
    ctr::println!("");
    ctr::println!("TID {}", reader.trainer_id());
}

pub fn draw_pkx(pkx: &Pk2) {
    ctr::println!("Species: {}", pkx.species);
    ctr::println!(color = get_shiny_color(pkx.shiny), "Shiny: {}", pkx.shiny);
    ctr::println!("HPower: {} {}", pkx.hidden_power_type, pkx.hidden_power_base);
    ctr::println!(color = get_iv_color(pkx.hp), "HP  DV: {}", pkx.hp);
    ctr::println!(color = get_iv_color(pkx.atk), "Atk DV: {}", pkx.atk);
    ctr::println!(color = get_iv_color(pkx.def), "Def DV: {}", pkx.def);
    ctr::println!(color = get_iv_color(pkx.spc), "Spc DV: {}", pkx.spc);
    ctr::println!(color = get_iv_color(pkx.spe), "Spe DV: {}", pkx.spe);
}

const DAYS: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];

pub fn draw_research(reader: &Gen2Reader, frame: usize) {
    let day = DAYS.get(reader.time_day() as usize).unwrap_or(&"Invalid");

    draw_rng(reader);
    ctr::println!("");
    ctr::println!("Frame {}", frame);
    ctr::println!("A Press {}", hid::is_pressing(Button::A));
    ctr::println!("DST {}", reader.dst());
    ctr::println!("Day {}", day);
    ctr::println!(
        "IGT {}:{:02}:{:02}",
        reader.time_hours(),
        reader.time_minutes(),
        reader.time_seconds()
    );
    ctr::println!(
        "Play time {}:{:02}:{:02}",
        reader.play_hours(),
        reader.play_minutes(),
        reader.play_seconds()
    );
}
