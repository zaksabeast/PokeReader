use core::convert::{TryFrom, TryInto};
use ctr::{
    pm_dbg,
    res::{GenericResultCode, ResultCode},
};
use num_enum::IntoPrimitive;

#[derive(Clone, Copy, Debug, PartialEq, IntoPrimitive)]
#[repr(u64)]
pub enum SupportedTitle {
    PokemonY = 0x0004000000055E00,
    PokemonUM = 0x00040000001B5100,
}

impl SupportedTitle {
    pub fn from_running_app() -> Option<Self> {
        pm_dbg::get_current_app_info()
            // Return None if there isn't a running app
            .ok()?
            .program_info
            .program_id
            .try_into()
            // Return None if the app isn't supported
            .ok()
    }
}

impl TryFrom<u64> for SupportedTitle {
    type Error = ResultCode;

    fn try_from(title_id: u64) -> Result<Self, Self::Error> {
        match title_id {
            0x0004000000055E00 => Ok(Self::PokemonY),
            0x00040000001B5100 => Ok(Self::PokemonUM),
            _ => Err(GenericResultCode::InvalidValue.into()),
        }
    }
}

impl PartialEq<u64> for SupportedTitle {
    fn eq(&self, other: &u64) -> bool {
        (*self as u64) == *other
    }
}
