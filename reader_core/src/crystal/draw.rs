use super::hook::{add_div_tracker, measured_div, rng_advance, sub_div_tracker};
use super::pk2::Pk2;
use super::reader::Gen2Reader;
pub use crate::draw::draw_header;
use crate::pnp::{self, Button};

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
        Some(index) => pnp::println!("ADIV Index {}", index),
        None => pnp::println!("Finding ADIV Index..."),
    };
    match sub_div_tracker().index() {
        Some(index) => pnp::println!("SDIV Index {}", index),
        None => pnp::println!("Finding SDIV Index..."),
    }
    pnp::println!("DIV {:04X}", measured_div());
    pnp::println!("State {:04X}", reader.rng_state());
    pnp::println!("Advances {}", rng_advance());
}

pub fn draw_pkx(pkx: &Pk2) {
    pnp::println!("Species: {}", pkx.species);
    pnp::println!(color = get_shiny_color(pkx.shiny), "Shiny: {}", pkx.shiny);
    pnp::println!(color = get_iv_color(pkx.hp), "HP  DV: {}", pkx.hp);
    pnp::println!(color = get_iv_color(pkx.atk), "Atk DV: {}", pkx.atk);
    pnp::println!(color = get_iv_color(pkx.def), "Def DV: {}", pkx.def);
    pnp::println!(color = get_iv_color(pkx.spc), "Spc DV: {}", pkx.spc);
    pnp::println!(color = get_iv_color(pkx.spe), "Spe DV: {}", pkx.spe);
}

pub fn draw_non_cfw(reader: &Gen2Reader, frame: usize) {
    draw_rng(reader);
    pnp::println!("");
    pnp::println!("Frame {}", frame);
    pnp::println!("A Press {}", pnp::is_pressing(Button::A));
}
