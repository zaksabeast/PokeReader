use crate::pkrd::reader::{Daycare, DaycareSlot};
use crate::pkrd::{display, views::view};
use crate::utils::daycare;
use ctr::res::CtrResult;

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

pub fn draw(screen: &mut display::DirectWriteScreen, daycare: &Daycare) -> CtrResult<()> {
    let parent1 = &daycare.parent_1;
    let parent2 = &daycare.parent_2;
    let is_masuda_method = daycare::is_daycare_masuda_method(parent1, parent2);

    view::draw_top_right(
        screen,
        daycare.daycare_title,
        &[
            &alloc::format!("Egg Ready: {}", daycare.is_egg_ready),
            &daycare::format_egg_parent(1, parent1),
            &daycare::format_egg_parent(2, parent2),
            "",
            &alloc::format!("Egg[1]: {:08X}", daycare.egg_seed[0]),
            &alloc::format!("Egg[0]: {:08X}", daycare.egg_seed[1]),
            "",
            &alloc::format!("Masuda Method: {}", is_masuda_method),
            daycare.daycare_footer,
        ],
    )?;

    Ok(())
}
