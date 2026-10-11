use super::title::{LoadedTitle, TitleError, loaded_title};
use crate::{
    GIT_HASH, VERSION, ctr,
    games::{crystal, gen6, gen7, transporter},
};

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
            ctr::println!("Unsupported game update!");
            ctr::println!("");
            ctr::println!("Please update your game");
            ctr::println!("for PokeReader to run");
            ctr::println!("");
            ctr::println!("Detected info:");
            ctr::println!(
                "Playing on {}",
                match is_citra {
                    true => "Citra",
                    false => "Real hardware",
                }
            );
            ctr::println!("Update ver {}", remaster_version);
            ctr::println!("Debug: {}", debug_info.as_deref().unwrap_or_default());
            ctr::println!("");
            ctr::println!("PokeReader version:");
            ctr::println!("{} {}", VERSION, GIT_HASH);
        }
        Err(TitleError::CouldNotGetProgramInfo) => {
            ctr::println!("Could not get program info!");
            ctr::println!("");
            ctr::println!("PokeReader version:");
            ctr::println!("{} {}", VERSION, GIT_HASH);
        }
        Err(TitleError::InvalidTitle) => {}
    }
}
