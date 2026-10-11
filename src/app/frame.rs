use super::title::{LoadedTitle, TitleError, loaded_title};
use crate::{GIT_HASH, VERSION, crystal, gen6, gen7, pnp, transporter};

fn run_loaded_title_frame(title: &LoadedTitle) {
    match title {
        LoadedTitle::S | LoadedTitle::M => gen7::run_sm_frame(),
        LoadedTitle::Us | LoadedTitle::Um => gen7::run_usum_frame(),
        LoadedTitle::Or | LoadedTitle::As => gen6::run_oras_frame(),
        LoadedTitle::X | LoadedTitle::Y => gen6::run_xy_frame(),
        LoadedTitle::Transporter => transporter::run_frame(),
        LoadedTitle::CrystalEn
        | LoadedTitle::CrystalDe
        | LoadedTitle::CrystalFr
        | LoadedTitle::CrystalEs
        | LoadedTitle::CrystalIt => crystal::run_frame(),
    }
}

pub fn run_frame() {
    match loaded_title() {
        Ok(title) => run_loaded_title_frame(title),
        Err(TitleError::InvalidUpdate {
            remaster_version,
            debug_info,
            is_citra,
        }) => {
            pnp::println!("Unsupported game update!");
            pnp::println!("");
            pnp::println!("Please update your game");
            pnp::println!("for PokeReader to run");
            pnp::println!("");
            pnp::println!("Detected info:");
            pnp::println!(
                "Playing on {}",
                match is_citra {
                    true => "Citra",
                    false => "Real hardware",
                }
            );
            pnp::println!("Update ver {}", remaster_version);
            pnp::println!("Debug: {}", debug_info.as_deref().unwrap_or_default());
            pnp::println!("");
            pnp::println!("PokeReader version:");
            pnp::println!("{} {}", VERSION, GIT_HASH);
        }
        Err(TitleError::CouldNotGetProgramInfo) => {
            pnp::println!("Could not get program info!");
            pnp::println!("");
            pnp::println!("PokeReader version:");
            pnp::println!("{} {}", VERSION, GIT_HASH);
        }
        Err(TitleError::InvalidTitle) => {}
    }
}
