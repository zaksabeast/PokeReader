use crate::pkrd::{display, display::Screen, reader, rng};
use ctr::{
    hid,
    hid::{Button, InterfaceDevice},
    res::CtrResult,
    safe_transmute,
};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Rng7View {
    is_active: bool,
}

impl Rng7View {
    pub fn get_is_active(&mut self) -> bool {
        if hid::Global::is_just_pressed(Button::Start | Button::Dup) {
            self.is_active = !self.is_active;
        }

        self.is_active
    }

    pub fn set_is_active(&mut self, is_active: bool) {
        self.is_active = is_active;
    }

    pub fn run_view(
        game: &impl reader::Gen7Reader,
        rng: &rng::Gen7Rng,
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
            screen.draw_string(&white, "Hello from rust!", x, y)?;

            y += 12;
            screen.draw_string(&white, "Official Luma", x, y)?;

            y += 12;
            screen.draw_string(&white, "Not NTR", x, y)?;

            y += 16;
            let init_seed = game.get_initial_seed();
            let seed_text = &alloc::format!("Init seed: {:08X}", init_seed);
            screen.draw_string(&white, seed_text, x, y)?;

            let sfmt_state = game.get_sfmt_state();
            let sfmt_state_bytes = sfmt_state.to_ne_bytes();
            let sfmt_state_parts: [u32; 2] =
                safe_transmute::transmute_one_pedantic(&sfmt_state_bytes)?;

            y += 12;
            let state_text = &alloc::format!("Curr state[1]: {:08X}", sfmt_state_parts[1]);
            screen.draw_string(&white, state_text, x, y)?;

            y += 12;
            let state_text = &alloc::format!("Curr state[0]: {:08X}", sfmt_state_parts[0]);
            screen.draw_string(&white, state_text, x, y)?;

            y += 12;
            let sfmt_advances = rng.get_sfmt_advances();
            let advances_text = &alloc::format!("Advances: {}", sfmt_advances);
            screen.draw_string(&white, advances_text, x, y)?;
        }

        Ok(())
    }
}
