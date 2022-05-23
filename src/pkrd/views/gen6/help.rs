use crate::pkrd::display::{DirectWriteScreen, Screen};
use crate::pkrd::views::view;
use ctr::res::CtrResult;

pub mod input {
    use ctr::hid::{Button, Global, InterfaceDevice};

    pub fn toggle() -> bool {
        Global::is_just_pressed(Button::X | Button::Dup)
    }
}

pub fn draw(screen: &mut DirectWriteScreen) -> CtrResult<()> {
    if !screen.get_is_top_screen() {
        view::draw_entire_bottom(
            screen,
            "PokeReader Gen 6 Help Menu",
            &[
                "",
                "X+Up: Show this menu",
                "Start+Up: Main RNG View",
                "Start+Down: Daycare View",
                "Start+Right: Party View",
                "Select+Left: Decrement current view",
                "Select+Right: Increment current view",
                "Start+Select: Pause game",
                "Pause+A: Unpause game",
                "Pause+Start: Unpause game",
                "Pause+Select: Advance one frame",
            ],
        )?;
    }
    Ok(())
}