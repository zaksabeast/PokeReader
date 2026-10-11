use crate::utils;

pub fn init_transporter() {
    utils::hook_gen6_mt_seed(0x117fdc);
}
