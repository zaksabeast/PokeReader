use crate::pnp;
use num_enum::TryFromPrimitive;

#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive)]
#[repr(u64)]
pub enum LoadedTitle {
    X = 0x0004000000055D00,
    Y = 0x0004000000055E00,
    Or = 0x000400000011C400,
    As = 0x000400000011C500,
    S = 0x0004000000164800,
    M = 0x0004000000175E00,
    Us = 0x00040000001B5000,
    Um = 0x00040000001B5100,
    Transporter = 0x00040000000C9C00,
    CrystalEn = 0x0004000000172800,
    CrystalDe = 0x0004000000172B00,
    CrystalFr = 0x0004000000172E00,
    CrystalEs = 0x0004000000173100,
    CrystalIt = 0x0004000000173400,
}

#[derive(Debug, Clone, Copy)]
pub enum TitleError {
    InvalidTitle,
    InvalidUpdate { remaster_version: u64 },
}

static mut LOADED_TITLE: Result<LoadedTitle, TitleError> = Err(TitleError::InvalidTitle);

pub fn loaded_title() -> Result<LoadedTitle, TitleError> {
    // Reader is single-threaded, so this is safe.
    // Even then, title and update version will also always be the same values.
    unsafe {
        let title = pnp::title_id()
            .try_into()
            .map_err(|_| TitleError::InvalidTitle)?;
        LOADED_TITLE = match (title, pnp::update_version()) {
            (LoadedTitle::S, 2)
            | (LoadedTitle::M, 2)
            | (LoadedTitle::Us, 2)
            | (LoadedTitle::Um, 2)
            | (LoadedTitle::Or, 7)
            | (LoadedTitle::As, 7)
            | (LoadedTitle::X, 5)
            | (LoadedTitle::Y, 5)
            | (LoadedTitle::Transporter, 5)
            | (LoadedTitle::CrystalEn, 0)
            | (LoadedTitle::CrystalDe, 0)
            | (LoadedTitle::CrystalFr, 0)
            | (LoadedTitle::CrystalEs, 0)
            | (LoadedTitle::CrystalIt, 0) => Ok(title),
            (_, remaster_version) => Err(TitleError::InvalidUpdate { remaster_version }),
        };

        LOADED_TITLE
    }
}
