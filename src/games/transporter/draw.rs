use super::rng::TransporterRng;
use crate::ctr;
pub use crate::draw::{PkxType, draw_header, draw_pkx};

pub fn draw_rng(rng: &TransporterRng) {
    let mt = rng.mt();
    ctr::println!("Init seed: {:08X}", mt.init_seed());
    ctr::println!("Curr state: {:08X}", mt.current_state());
    ctr::println!("MT Advances: {}", mt.advances());
}
