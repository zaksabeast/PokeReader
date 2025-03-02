use crate::pnp;

pub fn hook_gen6_mt_seed(mt_nop_addr: u32) {
    // The MT table init starts with a nop.
    // `str seed, [table, #-4]` can be used to persist the init seed 4 bytes before the MT table.
    pnp::write(mt_nop_addr, &0xe5001004u32);
}
