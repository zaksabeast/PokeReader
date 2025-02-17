use crate::pnp;
use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(u64)]
pub enum SupportedTitle {
    #[num_enum(default)]
    Invalid = 0,
    X = 0x0004000000055D00,
    Y = 0x0004000000055E00,
    Or = 0x000400000011C400,
    As = 0x000400000011C500,
    S = 0x0004000000164800,
    M = 0x0004000000175E00,
    Us = 0x00040000001B5000,
    Um = 0x00040000001B5100,
    Transporter = 0x00040000000C9C00,
    Crytal = 0x0004000000172800,
    CrystalFr = 0x0004000000172E00,
}

pub fn title_id() -> SupportedTitle {
    pnp::title_id().into()
}
