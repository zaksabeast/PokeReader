use super::title::LoadedTitle;
use crate::{crystal, gen6, gen7, transporter};

fn initialize_loaded_title(title: &LoadedTitle) {
    match title {
        LoadedTitle::S | LoadedTitle::M => gen7::init_sm(),
        LoadedTitle::Us => gen7::init_us(),
        LoadedTitle::Um => gen7::init_um(),
        LoadedTitle::Or | LoadedTitle::As => gen6::init_oras(),
        LoadedTitle::X | LoadedTitle::Y => gen6::init_xy(),
        LoadedTitle::Transporter => transporter::init_transporter(),
        LoadedTitle::CrystalEn
        | LoadedTitle::CrystalDe
        | LoadedTitle::CrystalFr
        | LoadedTitle::CrystalEs
        | LoadedTitle::CrystalIt => crystal::init_crystal(),
    }
}

pub fn init() {
    if let Ok(title) = super::title::loaded_title() {
        initialize_loaded_title(title);
    }
}
