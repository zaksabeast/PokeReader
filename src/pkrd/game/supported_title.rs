use core::convert::{TryFrom, TryInto};
use ctr::{
    pm_dbg,
    res::{GenericResultCode, ResultCode},
};
use num_enum::IntoPrimitive;

#[derive(Clone, Copy, Debug, PartialEq, IntoPrimitive)]
#[repr(u64)]
pub enum SupportedTitle {
    PokemonX = 0x0004000000055D00,
    PokemonY = 0x0004000000055E00,
    PokemonOR = 0x000400000011C400,
    PokemonAS = 0x000400000011C500,
    PokemonS = 0x0004000000164800,
    PokemonM = 0x0004000000175E00,
    PokemonUS = 0x00040000001B5000,
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
            0x0004000000055D00 => Ok(Self::PokemonX),
            0x0004000000055E00 => Ok(Self::PokemonY),
            0x000400000011C400 => Ok(Self::PokemonOR),
            0x000400000011C500 => Ok(Self::PokemonAS),
            0x0004000000164800 => Ok(Self::PokemonS),
            0x0004000000175E00 => Ok(Self::PokemonM),
            0x00040000001B5000 => Ok(Self::PokemonUS),
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
