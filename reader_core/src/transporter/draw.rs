use super::rng::TransporterRng;
pub use crate::draw::{draw_header, draw_pkx};
use crate::pnp;

pub fn draw_rng(rng: &TransporterRng) {
    let mt = rng.mt();
    pnp::println!("Init seed: {:08X}", mt.init_seed());
    pnp::println!("Curr state: {:08X}", mt.current_state());
    pnp::println!("MT Advances: {}", mt.advances());
}
