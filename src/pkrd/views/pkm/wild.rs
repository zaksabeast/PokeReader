use crate::pkrd::display;
use ctr::res::CtrResult;
use pkm_rs::pkm;

pub mod input {
    use ctr::hid::{Button, Global, InterfaceDevice};

    pub fn toggle() -> bool {
        Global::is_just_pressed(Button::Start | Button::Dleft)
    }
}

pub fn draw(screen: &mut display::DirectWriteScreen, pkx: &impl pkm::Pkx) -> CtrResult<()> {
    super::pkx::draw(screen, "Wild", pkx)
}
