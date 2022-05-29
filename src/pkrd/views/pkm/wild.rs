use crate::pkrd::{display, reader::Wild};
use ctr::res::CtrResult;

pub mod input {
    use ctr::hid::{Button, Global, InterfaceDevice};

    use crate::pkrd::reader::WildSlot;

    pub fn toggle() -> bool {
        Global::is_just_pressed(Button::Start | Button::Dleft)
    }

    fn increment() -> bool {
        Global::is_just_pressed(Button::Select | Button::Dright)
    }

    fn decrement() -> bool {
        Global::is_just_pressed(Button::Select | Button::Dleft)
    }

    pub fn next_wild_slot(mut slot: WildSlot) -> WildSlot {
        if increment() {
            slot.increment();
        }

        if decrement() {
            slot.decrement();
        }

        slot
    }
}

pub fn draw(screen: &mut display::DirectWriteScreen, wild: Wild) -> CtrResult<()> {
    super::pkx::draw(screen, wild.title, &wild.pkx)
}
