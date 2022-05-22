use alloc::string::String;
use pkm_rs::pkm;

pub fn is_masuda_method(parent1: &impl pkm::Pkx, parent2: &impl pkm::Pkx) -> bool {
    parent1.language() != parent2.language()
        && parent1.language() != pkm::Language::Invalid
        && parent2.language() != pkm::Language::Invalid
}

pub fn is_daycare_masuda_method(
    parent1: &Option<impl pkm::Pkx>,
    parent2: &Option<impl pkm::Pkx>,
) -> bool {
    match (parent1, parent2) {
        (Some(inner1), Some(inner2)) => is_masuda_method(inner1, inner2),
        (_, _) => false,
    }
}

pub fn format_egg_parent(parent_num: u8, parent: &Option<impl pkm::Pkx>) -> String {
    let formatted_parent = match parent {
        Some(parent) => alloc::format!(
            "Par{}: {} {}",
            parent_num,
            parent.species(),
            parent.gender_ratio()
        ),
        None => alloc::format!("Par{}: {}", parent_num, pkm::Species::None),
    };

    formatted_parent
}
