use crate::pnp;

pub fn init_transporter() {
    // The MT table init starts with a nop.
    // `str seed, [table, #-4]` can be used to persist the init seed 4 bytes before the MT table.
    pnp::write(0x117fdc, &0xe5001004u32);
}
