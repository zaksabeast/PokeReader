use crate::pnp;

pub fn init_xy() {
    // The MT table init starts with a nop.
    // `str seed, [table, #-4]` can be used to persist the init seed 4 bytes before the MT table.
    pnp::write(0x1254f8, &0xe5001004u32);
}

pub fn init_oras() {
    // The MT table init starts with a nop.
    // `str seed, [table, #-4]` can be used to persist the init seed 4 bytes before the MT table.
    pnp::write(0x125ec8, &0xe5001004u32);
}
