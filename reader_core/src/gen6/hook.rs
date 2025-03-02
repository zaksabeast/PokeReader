use crate::utils;

pub fn init_xy() {
    utils::hook_gen6_mt_seed(0x1254f8);
}

pub fn init_oras() {
    utils::hook_gen6_mt_seed(0x125ec8);
}
