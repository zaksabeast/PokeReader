use super::hook::{add_div_tracker, measured_div, rng_advance, sub_div_tracker};
use super::pk2::Pk2;
use super::reader::Gen2Reader;
pub use crate::draw::draw_header;
use crate::pnp::{self, Button};

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
    pnp::println!("Shiny: {}", pkx.shiny);
    pnp::println!(
        "DVs: {}/{}/{}/{}/{}",
        pkx.hp,
        pkx.atk,
        pkx.def,
        pkx.spc,
        pkx.spe,
    );
}

pub fn draw_non_cfw(reader: &Gen2Reader, frame: usize) {
    draw_rng(reader);
    pnp::println!("");
    pnp::println!("Frame {}", frame);
    pnp::println!("A Press {}", pnp::is_pressing(Button::A));
}
