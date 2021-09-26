use crate::pkrd::{display, reader::pkm};
use ctr::{
    hid,
    hid::{Button, InterfaceDevice},
    res::CtrResult,
};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct PartyView {
    is_active: bool,
    slot: pkm::PartySlot,
}

impl PartyView {
    pub fn get_is_active(&mut self) -> bool {
        if hid::Global::is_just_pressed(Button::Start | Button::Dright) {
            self.is_active = !self.is_active;
        }

        if hid::Global::is_just_pressed(Button::Select | Button::Dright) {
            self.slot = self.slot.increment()
        }

        if hid::Global::is_just_pressed(Button::Select | Button::Dleft) {
            self.slot = self.slot.decrement()
        }

        self.is_active
    }

    pub fn set_is_active(&mut self, is_active: bool) {
        self.is_active = is_active;
    }

    pub fn run_view(
        &mut self,
        pkx: &impl pkm::Pkx,
        screen: &mut display::DirectWriteScreen,
    ) -> CtrResult<()> {
        let title = &alloc::format!("Party {}", self.slot);
        super::pkx::run_view(title, pkx, screen)?;

        Ok(())
    }

    pub fn get_slot(&self) -> pkm::PartySlot {
        self.slot
    }
}
