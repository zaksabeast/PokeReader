use crate::pkrd::{display, display::Screen, reader};
use ctr::{
    hid,
    hid::{Button, InterfaceDevice},
    res::CtrResult,
};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Daycare7View {
    is_active: bool,
}

impl Daycare7View {
    pub fn get_is_active(&mut self) -> bool {
        if hid::Global::is_just_pressed(Button::Start | Button::Ddown) {
            self.is_active = !self.is_active;
        }

        self.is_active
    }

    pub fn set_is_active(&mut self, is_active: bool) {
        self.is_active = is_active;
    }

    pub fn run_view(
        game: &impl reader::Gen7Reader,
        screen: &mut display::DirectWriteScreen,
    ) -> CtrResult<()> {
        if screen.get_is_top_screen() {
            let mut x = 200;
            let mut y = 10;

            let black = display::Color::black();
            let white = display::Color::white();

            screen.paint_square(&black, x, y, 192, 92)?;

            x += 4;
            y += 4;
            screen.draw_string(&white, "Daycare View", x, y)?;

            y += 16;
            let egg_seed = game.get_egg_seed();
            screen.draw_string(&white, &alloc::format!("Egg[0]: {:08X}", egg_seed[3]), x, y)?;

            y += 12;
            screen.draw_string(&white, &alloc::format!("Egg[1]: {:08X}", egg_seed[2]), x, y)?;

            y += 12;
            screen.draw_string(&white, &alloc::format!("Egg[2]: {:08X}", egg_seed[1]), x, y)?;

            y += 12;
            screen.draw_string(&white, &alloc::format!("Egg[3]: {:08X}", egg_seed[0]), x, y)?;
        }

        Ok(())
    }
}
