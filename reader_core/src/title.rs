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
    InvalidUpdate { remaster_version: u16, is_citra: bool },
}

static mut LOADED: bool = false;
static mut LOAD_RESULT: Result<LoadedTitle, TitleError> = Err(TitleError::InvalidTitle);

// Older citra builds can't reliably report the game's version
// via the fs sysmodule. For that environment we detect whether
// the running game is the latest version by checking a known, unique
// 16-byte value located at a fixed address in the game's memory.
fn check_citra_title_version(addr: u32, expected: &'static [u8; 16], version: u16) -> u16 {
    match pnp::read_array::<16>(addr) == *expected {
        true => version,
        false => 0,
    }
}

fn get_citra_title_version(title: LoadedTitle) -> u16 {
    match title {
        LoadedTitle::S => check_citra_title_version(0x3d3a90, b"8QjtffIMWFhiFpTz", 2),
        LoadedTitle::M => check_citra_title_version(0x3d3a90, b"7mXz0DXR4b4CdD8r", 2),
        LoadedTitle::Us => check_citra_title_version(0x3e5888, b"fnCAH3KrGIl9dgSd", 2),
        LoadedTitle::Um => check_citra_title_version(0x3e5888, b"b3Gq6LF6EqE1bvKy", 2),
        LoadedTitle::Or => check_citra_title_version(0x1086bc, b"cRFY0WFHNjPh44If", 7),
        LoadedTitle::As => check_citra_title_version(0x1086bc, b"guBwm9TlQvYvncKn", 7),
        LoadedTitle::X => check_citra_title_version(0x10869c, b"h0VRqB2YEgq39zvO", 5),
        LoadedTitle::Y => check_citra_title_version(0x10869c, b"Slv7vHlUOfqrKMpz", 5),
        LoadedTitle::Transporter => 5,
        LoadedTitle::CrystalEn
        | LoadedTitle::CrystalDe
        | LoadedTitle::CrystalFr
        | LoadedTitle::CrystalEs
        | LoadedTitle::CrystalIt => 0,
    }
}

fn get_update_version(title: LoadedTitle) -> u16 {
    if pnp::is_citra() {
        return get_citra_title_version(title);
    }

    pnp::update_version()
}

pub fn loaded_title() -> Result<LoadedTitle, TitleError> {
    // Reader is single-threaded, so this is safe.
    // Even then, title and update version will also always be the same values.
    unsafe {
        if LOADED {
            return LOAD_RESULT;
        }

        LOADED = true;

        let title = match pnp::title_id().try_into() {
            Ok(title) => title,
            Err(_) => {
                LOAD_RESULT = Err(TitleError::InvalidTitle);
                return LOAD_RESULT;
            }
        };

        LOAD_RESULT = match (title, get_update_version(title)) {
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
            (_, remaster_version) => Err(TitleError::InvalidUpdate {
                remaster_version,
                is_citra: pnp::is_citra(),
            }),
        };

        LOAD_RESULT
    }
}
