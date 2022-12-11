use pkm_rs::{types, Pkx};

pub fn is_masuda_method(parent1: &impl Pkx, parent2: &impl Pkx) -> bool {
    parent1.language() != parent2.language()
        && parent1.language() != types::Language::Invalid
        && parent2.language() != types::Language::Invalid
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
            parent.species(),
            parent.gender_ratio()
        ),
        None => format!("Par{}: {}", parent_num, types::Species::None),
    }
}
