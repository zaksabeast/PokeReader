use crate::pkrd::{display, reader::pkm};
use ctr::{
    hid,
    hid::{Button, InterfaceDevice},
    res::CtrResult,
};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct PartyView {
    show: bool,
    slot: pkm::PartySlot,
}

impl PartyView {
    pub fn run_view(
        &mut self,
        party_pkx: &impl pkm::Pkx,
        screen: &mut display::DirectWriteScreen,
    ) -> CtrResult<()> {
        if hid::Global::is_just_pressed(Button::Start | Button::Dright) {
            self.show = !self.show;
        }

        if hid::Global::is_just_pressed(Button::Select | Button::Dright) {
            self.slot = self.slot.increment()
        }

        if hid::Global::is_just_pressed(Button::Select | Button::Dleft) {
            self.slot = self.slot.decrement()
        }

        if self.show {
            let title = &alloc::format!("Party {}", self.slot);
            super::pkx::run_view(title, party_pkx, screen)?;
        }

        Ok(())
    }

    pub fn get_slot(&self) -> pkm::PartySlot {
        self.slot
    }
}
