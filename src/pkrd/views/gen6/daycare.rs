use crate::pkrd::reader::{Daycare, DaycareSlot};
use crate::pkrd::{display, display::Screen, views::view};
use alloc::string::String;
use ctr::res::CtrResult;
use pkm_rs::pkm;

pub mod input {
    use super::*;
    use ctr::hid::{Button, Global, InterfaceDevice};

    pub fn toggle() -> bool {
        Global::is_just_pressed(Button::Start | Button::Ddown)
    }

    fn increment() -> bool {
        Global::is_just_pressed(Button::Select | Button::Dright)
    }

    fn decrement() -> bool {
        Global::is_just_pressed(Button::Select | Button::Dleft)
    }

    pub fn next_daycare_slot(mut slot: DaycareSlot) -> DaycareSlot {
        if increment() {
            slot.increment();
        }

        if decrement() {
            slot.decrement();
        }

        slot
    }
}

fn is_masuda_method(parent1: &impl pkm::Pkx, parent2: &impl pkm::Pkx) -> bool {
    parent1.language() != parent2.language()
        && parent1.language() != pkm::Language::Invalid
        && parent2.language() != pkm::Language::Invalid
}

fn is_daycare_masuda_method(
    parent1: &Option<impl pkm::Pkx>,
    parent2: &Option<impl pkm::Pkx>,
) -> bool {
    match (parent1, parent2) {
        (Some(inner1), Some(inner2)) => is_masuda_method(inner1, inner2),
        (_, _) => false,
    }
}

fn format_egg_parent(parent_num: u8, parent: &Option<impl pkm::Pkx>) -> String {
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

pub fn draw(screen: &mut display::DirectWriteScreen, daycare: &Daycare) -> CtrResult<()> {
    if screen.get_is_top_screen() {
        let is_egg_ready = daycare.is_egg_ready;
        let parent1 = &daycare.parent_1;
        let parent2 = &daycare.parent_2;
        let egg_seed = daycare.egg_seed;
        let is_masuda_method = is_daycare_masuda_method(&parent1, &parent2);

        view::draw_right(
            screen,
            daycare.daycare_title,
            &[
                &alloc::format!("Egg Ready: {}", is_egg_ready),
                &format_egg_parent(1, &parent1),
                &format_egg_parent(2, &parent2),
                "",
                &alloc::format!("Egg[0]: {:08X}", egg_seed[1]),
                &alloc::format!("Egg[1]: {:08X}", egg_seed[0]),
                "",
                &alloc::format!("Masuda Method: {}", is_masuda_method),
                daycare.daycare_footer,
            ],
        )?;
    }

    Ok(())
}
