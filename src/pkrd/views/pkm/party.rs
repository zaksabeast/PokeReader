use crate::{pkrd::display, utils::party_slot::PartySlot};
use ctr::res::CtrResult;
use pkm_rs::pkm;

pub mod input {
    use super::*;
    use ctr::hid::{Button, Global, InterfaceDevice};

    pub fn toggle() -> bool {
        Global::is_just_pressed(Button::Start | Button::Dright)
    }

    fn increment() -> bool {
        Global::is_just_pressed(Button::Select | Button::Dright)
    }

    fn decrement() -> bool {
        Global::is_just_pressed(Button::Select | Button::Dleft)
    }

    pub fn next_party_slot(mut slot: PartySlot) -> PartySlot {
        if increment() {
            slot.increment();
        }

        if decrement() {
            slot.decrement();
        }

        slot
    }
}

pub fn draw(
    screen: &mut display::DirectWriteScreen,
    pkx: &impl pkm::Pkx,
    slot: PartySlot,
) -> CtrResult<()> {
    let title = &alloc::format!("Party {}", slot);
    super::pkx::draw(screen, title, pkx)
}
