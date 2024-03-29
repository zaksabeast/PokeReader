use alloc::{format, string::String};
use pkm_rs::{Language, Pkx, Species};

pub fn is_masuda_method(parent1: &impl Pkx, parent2: &impl Pkx) -> bool {
    parent1.language_t() != parent2.language_t()
        && parent1.language_t() != Language::Invalid
        && parent2.language_t() != Language::Invalid
}

pub fn is_daycare_masuda_method(parent1: &Option<impl Pkx>, parent2: &Option<impl Pkx>) -> bool {
    match (parent1, parent2) {
        (Some(inner1), Some(inner2)) => is_masuda_method(inner1, inner2),
        (_, _) => false,
    }
}

pub fn format_egg_parent(parent_num: u8, parent: &Option<impl Pkx>) -> String {
    match parent {
        Some(parent) => format!(
            "Par{}: {} {}",
            parent_num,
            parent.species_t(),
            parent.gender_ratio()
        ),
        None => format!("Par{}: {}", parent_num, Species::None),
    }
}
