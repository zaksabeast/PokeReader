use crate::pkrd::display;
use ctr::{
    hid,
    hid::{Button, InterfaceDevice},
    res::CtrResult,
};
use pkm_rs::pkm;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct WildView {
    is_active: bool,
}

impl WildView {
    pub fn get_is_active(&mut self) -> bool {
        if hid::Global::is_just_pressed(Button::Start | Button::Dleft) {
            self.is_active = !self.is_active;
        }

        self.is_active
    }

    pub fn set_is_active(&mut self, is_active: bool) {
        self.is_active = is_active;
    }

    pub fn run_view(pkx: &impl pkm::Pkx, screen: &mut display::DirectWriteScreen) -> CtrResult<()> {
        super::pkx::run_view("Wild", pkx, screen)?;

        Ok(())
    }
}
